# Criptografia RSA

## Descrição

Este é um programa em Rust que implementa o algoritmo de criptografia RSA. RSA é um dos primeiros algoritmos de criptografia de chave pública e é amplamente utilizado para comunicações seguras na Internet, incluindo transações online e autenticação de usuários.

O programa consiste em quatro funções principais que são recebidas através de um arquivo de texto que é chamado como argumento do programa. Essas funções permitem gerar pares de chaves RSA, remover chaves existentes, criptografar mensagens ASCII e decodificar mensagens criptografadas.

## Compilação

Para compilar e rodar o programa de criptografia RSA em Rust, siga os passos abaixo:

### Instalação do Cargo

Cargo é o sistema de build e gerenciamento de pacotes do Rust. Para instalar o Cargo e o Rust, siga estas instruções:

1. **Instalando o Rustup**: Rustup é uma ferramenta de linha de comando para gerenciar as instalações do Rust. Ele facilita a instalação e a atualização do Rust e do Cargo.

   - No Linux ou macOS, abra um terminal e execute:
     ```sh
     curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
     ```

   - No Windows, baixe e execute o instalador do Rust disponível em: [Rustup - Instalação](https://rustup.rs/)

2. **Siga as instruções do instalador**: Após executar o comando acima, siga as instruções na tela para completar a instalação. Isso inclui adicionar o Cargo e o Rust ao seu PATH.

3. **Verifique a instalação**: Após a instalação, feche e reabra o terminal, então verifique se o Cargo foi instalado corretamente executando:
   ```sh
   cargo --version

## Funcionalidades

### GEN_CHAVE

A função `GEN_CHAVE` é utilizada para gerar um novo par de chaves RSA.

**Chamada:**
```
GEN_CHAVE <nb> <ind> <dir>
```

- `nb`: Número de bits que a chave terá.
- `ind`: Identificador do par de chaves.
- `dir`: Diretório onde a chave RSA será armazenada. Se for no diretório atual, use "/".

### REM_CHAVE

A função `REM_CHAVE` remove um par de chaves existentes.

**Chamada:**
```
REM_CHAVE <ind> <dir>
```

- `ind`: Identificador do par de chaves a ser removido.
- `dir`: Diretório onde a chave RSA está armazenada. Se for no diretório atual, use "/".

### CRIPT_MSG

A função `CRIPT_MSG` é usada para criptografar uma mensagem ASCII.

**Chamada:**
```
CRIPT_MSG <cpub> <in> <out> <manter in>
```

- `cpub`: Caminho para a chave pública.
- `in`: Caminho para o arquivo contendo a mensagem ASCII.
- `out`: Caminho para o arquivo onde a mensagem criptografada será salva.
- `manter in`: 'n' se o arquivo original deve ser apagado após o processo.

### DCRIP_MSG

A função `DCRIP_MSG` decodifica uma mensagem criptografada.

**Chamada:**
```
DRIP_MSG <cpriv> <in> <out> <manter in>
```

- `cpriv`: Caminho para a chave privada.
- `in`: Caminho para o arquivo contendo a mensagem criptografada.
- `out`: Caminho para o arquivo onde a mensagem decodificada será salva.
- `manter in`: 'n' se o arquivo original deve ser apagado após o processo.

## Nota

Certifique-se de seguir corretamente a sintaxe e os parâmetros de cada função para garantir o funcionamento adequado do programa.
