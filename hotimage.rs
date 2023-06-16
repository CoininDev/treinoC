extern crate image;
use image::{Rgb, ImageBuffer};

fn ler_texto(caminho: String) -> String{
    let mut arquivo = File::open(caminho).expect("erro ao abrir arquivo");
    let mut texto = String::new();
    arquivo.read_to_string(&mut texto).expect("erro ao ler arquivo");
    return texto
}

fn formatar_info(bruto: String) -> Vec<Vec<f32>>{
    let texto: String = bruto.replace("0.00,", "").replace("nan,", "0.00,");
    let valores: Vec<f32>= texto.split(",").filter_map(|s| s.parse().ok()).collect();

    //transformar em matriz
    let largura = 17;

    let mut matriz = Vec::new();
    let mut linha = Vec::new();
    for valor in valores{
        linha.push(valor);

        if linha.len() == largura{
            matriz.push(linha);
            linha = Vec::new();
        }
    }
    return matriz
}

fn gerar_imagem(matriz: Vec<Vec<f32>>) -> ImageBuffer<Rgb<u8>, Vec<u8>>{
    let largura = matriz[0].len();
    let altura = matriz.len();

    let mut imagem = ImageBuffer::new(largura as u32, altura as u32);

    for (y, linha) in matriz.iter().enumerate(){
        for (x, temperatura) in linha.iter().enumerate(){
            //converte temperatura (Cº) para cor, 0=0, 50=255
            let temperaturai32: i32 = temperatura.floor() as i32 * 100; 
            
            //      atual máximo
            //temp.  18    50
            //cor    x    255
            // x = (18*255)/50
            // multiplicado por 100 para contar 2 casas decimais na hora de pintar o pixel, 18.25 != 18.82
            let vermelho = ((temperaturai32 * 255 ) / 5000) as u8;
            
            imagem.put_pixel(x as u32, y as u32, Rgb([vermelho, 5, 5]));
        }
    }

    return imagem;
}

fn main(){
    let info = formatar_info(ler_texto("input.txt"));
    let _ = gerar_imagem(info).save("output.png").unwrap();
}
