# rust_exemplos

# regras

\*\*\* Cada parâmetro que é uma referência tem seu próprio parâmetro de tempo de vida. Em outras palavras, uma função com um parâmetro tem um parâmetro de tempo de vida: fn foo<'a>(x: &'a i32), uma função com dois argumentos recebe dois parâmetros de tempo de vida separados: fn foo<'a, 'b>(x: &'a i32, y: &'b i32), e assim por diante.

\*\*\* Se há exatamente uma entrada de parâmetro de tempo de vida, aquele tempo de vida é atribuído para todos os parâmetros de saída do tempo de vida: fn foo<'a>(x: &'a i32) -> &'a i32.

\*\*\* Se há múltiplas entradas de parâmetros de tempo de vida, mas uma delas é &self ou &mut self porque é um método, então o tempo de vida de self é atribuído para todos os parâmetro de tempo de vida de saída. Isso melhora a escrita de métodos
