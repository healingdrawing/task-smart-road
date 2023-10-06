use super::Road;

impl<'a> Road<'a> {
  /** animate autos */
  pub fn animate_step(&mut self) {
    self.autos.ne.iter_mut().for_each(|auto| auto.animate_step());
    self.autos.nn.iter_mut().for_each(|auto| auto.animate_step());
    self.autos.nw.iter_mut().for_each(|auto| auto.animate_step());
    
    self.autos.sw.iter_mut().for_each(|auto| auto.animate_step());
    self.autos.ss.iter_mut().for_each(|auto| auto.animate_step());
    self.autos.se.iter_mut().for_each(|auto| auto.animate_step());

    self.autos.wn.iter_mut().for_each(|auto| auto.animate_step());
    self.autos.ww.iter_mut().for_each(|auto| auto.animate_step());
    self.autos.ws.iter_mut().for_each(|auto| auto.animate_step());

    self.autos.es.iter_mut().for_each(|auto| auto.animate_step());
    self.autos.ee.iter_mut().for_each(|auto| auto.animate_step());
    self.autos.en.iter_mut().for_each(|auto| auto.animate_step());
  }
}