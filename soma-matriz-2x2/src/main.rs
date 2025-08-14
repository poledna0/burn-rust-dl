// caso de algum erro do tipo recursive type evaluation tem add no inicio da main
//#![recursion_limit = "256"]

use burn::tensor::Tensor;
use burn::backend::Wgpu;

// apenas um alias para dps trocar o bkend de g
type Backend = Wgpu;

fn main() {
    let device = Default::default();
    //Tensor::<Backend, 2>::from_data([[2., 3.], [4., 5.]], &device);
    //             |     |              
    //  tipo generico    tensor 2D
    //  from_data é um método utilizado para criar o tensor a partir de dados brutos, no caso, uma matriz 2x2
    // device é a palca
    let tensor_1 = Tensor::<Backend, 2>::from_data([[2., 3.], [4., 5.]], &device);
    // cria a msm coisa q o tensor 1, mas a diferenca q o ones_like (parametro de copia) ele copia a msm matriz ou tensor e suas medidas só troca o valor para 1
    let tensor_2 = Tensor::<Backend, 2>::ones_like(&tensor_1);
    // soma cada linha e coluna sozinha com 1
    println!("{}", tensor_1 + tensor_2);
    
    /*
    Saida 
    Tensor {
  data:
[[3.0, 4.0],
 [5.0, 6.0]],
  shape:  [2, 2],
  device:  DefaultDevice,
  backend:  "fusion<cubecl<wgpu<wgsl>>>",
  kind:  "Float",
  dtype:  "f32",
}

     */
}
