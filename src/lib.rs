#![feature(external_doc)]
use sma_rs::SMA;
use ta_common::traits::Indicator;
use ta_common::fixed_queue::FixedQueue;
#[doc(include = "../README.md")]
pub struct MD {
    period: u32,
    sma: SMA,
    history: FixedQueue<f64>,
}

impl MD {
    pub fn new(period: u32) -> MD {
        Self {
            period,
            sma: SMA::new(period),
            history: FixedQueue::new(period),
        }
    }
}

impl Indicator<f64, Option<f64>> for MD {
    fn next(&mut self, input: f64) -> Option<f64> {
        self.history.add(input);
        let sma = self.sma.next(input).unwrap();
        return if self.history.is_full() {
            let mut sum_md = 0.0;
            for i in 0..self.history.size() as i32 {
                let p = self.history.at(i).unwrap();
                sum_md = sum_md + (p - sma).abs();
            }
            Some(sum_md / self.period as f64)
        } else {
            None
        };
    }

    fn reset(&mut self) {
        self.sma.reset();
    }
}


#[cfg(test)]
mod tests {
    use crate::MD;
    use ta_common::traits::Indicator;

    #[test]
    fn it_works() {
        let mut md = MD::new(5);
        assert_eq!(md.next(81.59), None);
        assert_eq!(md.next(81.06), None);
        assert_eq!(md.next(82.87), None);
        assert_eq!(md.next(83.00), None);
        assert_eq!(md.next(83.61), Some(0.8807999999999993));
        assert_eq!(md.next(83.15), Some(0.6712000000000102));
        assert_eq!(md.next(82.84), Some(0.2287999999999954));
        assert_eq!(md.next(83.99), Some(0.3855999999999938));
        assert_eq!(md.next(84.55), Some(0.5135999999999938));
        assert_eq!(md.next(84.36), Some(0.6263999999999982));
        assert_eq!(md.next(85.53), Some(0.6712000000000046));
        assert_eq!(md.next(86.54), Some(0.8327999999999974));
        assert_eq!(md.next(86.89), Some(0.9127999999999957));
        assert_eq!(md.next(87.77), Some(1.0184000000000082));
        assert_eq!(md.next(87.29), Some(0.6152000000000072));
    }
}
