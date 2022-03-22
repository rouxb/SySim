
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll, Waker};
use once_cell::sync::OnceCell;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc::{Receiver,Sender};

// WaitKind -------------------------------------------------------------------
/// WaitKind define the wait type use for the current job
#[derive(Debug)]
pub enum WaitKind {
    Time(usize),
    Event(String),
}

// HwJob ----------------------------------------------------------------------
/// HwJob: Structure use to pass wakeup information to the HwScheduler
#[derive(Debug)]
pub struct HwJob<'a> {
    wait_evt: WaitKind,
    waker: &'a Waker,
}

// HwScheduler ----------------------------------------------------------------
/// HwScheduler: Implement a async task that handle the simulated time update 
/// and the inter-task events.
/// Warn: This struct is instanciated once and made global with once_cell
#[derive(Debug)]
pub struct HwScheduler<'a> {
    tick: AtomicUsize,
    timescale: usize,
    hwRx: Receiver<HwJob<'a>>,
    inflight_hwt: AtomicUsize,
    pending_hwt: Vec<HwJob<'a>>,
    waker: Option<Waker>,
}
static TICK_KEEPER: OnceCell<HwScheduler> = OnceCell::new();

impl<'a> HwScheduler<'a> {
    /// Constructs a new `HwScheduler`.
    ///
    /// `cur_tick` Start from the given tick
    /// `timescale` Used time resolution
    ///
    pub fn new(cur_tick: usize, timescale: usize, rx: Receiver<HwJob>) -> Self {
        println!("Create a new HwScheduler @{}[{}]", cur_tick, timescale);
        HwScheduler {
            tick: AtomicUsize::new(cur_tick),
            timescale: timescale,
            hwRx: rx,
            inflight_hwt: AtomicUsize::new(0),
            pending_hwt: Vec::new(),
            waker: None,
        }
    }

    pub fn register(self: Self) -> () {
        TICK_KEEPER.set(self).unwrap();
    }

    pub fn global() -> &'static HwScheduler<'a> {
        TICK_KEEPER.get().expect("HwScheduler is not initialized")
    }

    pub fn simulate(self: &mut Self, duration: usize) -> () {
        println!("Start simulation loop for {} tick", duration);

        loop {
            todo!();
            // Read message from mpsc
            // get next tick
            // if next tick >= duration break
            // => break;
            // else update tick and wake required tasks
        };
    }

    /// Generate the given event id at tick
    pub fn notify(self: &mut Self, name: String, tick: usize) -> () {
        println!(" Event {} fired, notify associated HwTasks", name);
        todo!();
    }
}

/// Global scope function to retrieved some scheduler informations
pub fn cur_tick() -> usize {
    let tk = HwScheduler::global();
    tk.tick.load(Ordering::SeqCst)
}

// HwTask ---------------------------------------------------------------------
/// HwTask: Implement a async task that represent Hw component execution loop
/// In practice this should be a trait implemented by multiple component structures
///
#[derive(Debug)]
pub struct HwTask<'a> {
    name: String,
    kind: WaitKind,
    hw_tx: Sender<HwJob<'a>>,
}

impl<'a> HwTask<'a> {
    pub fn new(name: String, kind: WaitKind, tx: Sender<HwJob>) -> Self {
        println!("{}: Create HwTask {}[{:?}]", cur_tick(), name, kind);

        HwTask {
            name: name,
            kind: kind,
            hw_tx: tx,
        }
    }

    async fn run(self: &mut Self) -> () {
        loop {
            HwFuture::new(self).await;
            println!("{}: Execute HwTask {}[{:?}]", cur_tick(), self.name, self.kind);
        }
    }
}

// HwFuture ------------------------------------------------------------------
/// Implement a future that registered itself in the HwScheduler for waking up
/// bypass the standard tokio wakeup mechanisms for Hw simulation purpose
///
pub struct HwFuture<'p> {
    parent: &'p HwTask<'p>,
}

impl<'p> HwFuture<'p> {
    pub fn new(parent: &'p mut HwTask) -> Self {
        HwFuture {
            parent: parent,
        }
    }

    async fn wait_for(tick: usize) -> () {
        todo!()
    }

    async fn wait_event(name: &str) -> () {
        todo!()
    }
}

impl<'p> Future for HwFuture<'p> {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>)
        -> Poll<()>
    {
        println!("HwFuture polled");

        let ltick = cur_tick();
        match self.parent.kind {
            WaitKind::Time(t) => {
                // Ready path
                if ltick >= t {
                    Poll::Ready(())
                } else {
                    let job = HwJob {
                        wait_evt: WaitKind::Time(t),
                        waker: cx.waker()
                    };
                    self.parent.hw_tx.send(job);
                    // cx.waker().wake_by_ref();
                    Poll::Pending
                }
            },
            WaitKind::Event(n) => {
                todo!();
            },
            _ => { panic!("Unknown kind"); }
        }
    }
}
