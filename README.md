# Libris 1.0.7

Sistema escolar de biblioteca da CREDE, feito com HTML, CSS e JavaScript. Os dados operacionais ficam no IndexedDB da máquina da biblioteca.

## Estado da arquitetura

O painel usa IndexedDB para guardar livros, leitores, empréstimos, contas locais, configurações e auditoria. Os dados operacionais ficam no computador da escola. O Firebase Authentication é usado somente para o acesso restrito da CREDE e o Google é solicitado separadamente para enviar backups ao Drive.

## Acesso local

- A CREDE abre **Configuração local (CREDE)**, cadastra a escola e cria os emails e senhas dos usuários. Senhas são armazenadas como hashes PBKDF2 com salt, não em texto puro.
- Os usuários entram pela única tela normal de login. Não há criação nem redefinição de senha pelo usuário; a CREDE altera as credenciais na configuração local.
- O login do painel não usa conta Google. O Google aparece somente quando o usuário inicia o envio de backup ao Drive.
- A sessão permanece salva neste computador até a pessoa sair. Limpar os dados do navegador/aplicativo encerra a sessão e exige restaurar um backup.
- O botão **Importar** aceita um CSV legado de livros ou um ZIP completo de backup com arquivos CSV. Categorias ausentes do CSV legado são criadas usando o prefixo do código do livro; a escola de origem é remapeada para a escola configurada.
- O botão **Exportar** gera um backup completo ZIP com CSVs. O botão **Backup no Drive** envia o mesmo formato e exibe um lembrete semanal; o envio exige que o Libris esteja aberto e seja iniciado por alguém. Os CSVs não incluem senhas nem hashes de senha; a restauração mantém as contas locais já cadastradas no computador.
- A cota de 10 emails por escola/mês é contada no armazenamento local daquela instalação. Os livros com maior atraso aparecem primeiro; avisos de alunos por email são encaminhados à coordenação. A cota não é sincronizada entre vários computadores da mesma escola.

Mantenha cópias dos arquivos antigos até confirmar a importação do CSV e testar a restauração do backup completo.

## Configuração Firebase

1. No Firebase Console do projeto `librisce`, habilite **Authentication > Email/Password** para a conta administrativa CREDE. Adicione o domínio HTTPS publicado em **Authorized domains**.
2. Em **Authentication > Users**, crie `admin@crede17.ce.gov.br` com uma senha nova, forte e exclusiva. A senha digitada no app é validada pelo Firebase Authentication; ela não está no código. Como a senha anterior foi compartilhada em conversa, não a reutilize.
3. Ative a Google Drive API no Google Cloud do projeto. O app solicita o escopo `drive.file` somente quando alguém usa o botão de backup.
4. No computador da biblioteca, use **Configuração local (CREDE)** e entre com `admin@crede17.ce.gov.br` e a senha criada no Firebase.
5. Cadastre a escola, o email da coordenação, as contas locais e os parâmetros EmailJS. Os valores iniciais são `service_Libris`, `template_LibrisA`, `template_LibrisC` e a chave pública `3NnWny8jJqnZwiFWU`.

As contas locais da escola são independentes do Firebase Authentication; a senha administrativa CREDE continua sendo gerenciada pelo Firebase.

## Migração de dados legados

Para trazer o acervo do site antigo, selecione o CSV em **Importar**. Os nomes de categorias do arquivo são preservados e categorias que ainda não existem são cadastradas automaticamente com base no prefixo do código. IDs de escola antigos, como `escola_1`, são tratados como origem e substituídos pelo identificador local configurado. Para restaurar um backup Libris completo, selecione o ZIP gerado em **Exportar** ou enviado ao Drive; a restauração substitui os dados locais da escola atual.

## Backup semanal no Google Drive

No painel escolar, clique em **Backup no Drive** quando o status indicar que o backup semanal está pendente. O arquivo ZIP contém um CSV por conjunto de dados e vai para a conta Google escolhida na autorização. O primeiro envio pede consentimento `drive.file`; a conta Libris não recebe esse escopo ao apenas configurar a escola.

O navegador só envia enquanto o app está aberto. Se a máquina ficar desligada ou o app fechado na data prevista, o backup fica pendente até a próxima abertura e ação do usuário.

## Capas, sinopses e avisos por e-mail

Ao cadastrar um livro, o sistema busca automaticamente capa e descrição pelo título, autor e editora. A primeira fonte é o Google Books; se não houver resultado, ele tenta a Open Library. Se nenhuma fonte encontrar o livro, o cadastro continua normalmente sem esses dados.

Os avisos são enviados pelo EmailJS após ação do administrador, não em segundo plano. Para alunos, o email é encaminhado automaticamente à coordenação; para os demais tipos de leitor, vai ao email cadastrado. Há limite local de 10 mensagens por mês por escola e o painel prioriza visualmente os empréstimos com maior atraso. Esse limite não pode ser compartilhado entre computadores sem um serviço central. Os templates devem aceitar `to_email`, `to_name`, `subject`, `message`, `school_name`, `organization`, `livro`, `loan_days`, `data_vencimento` e `dias_atraso`.

## Executar localmente

Gere os arquivos publicados e sirva a pasta `dist` por HTTP:

```sh
npm run build:hosting
python3 -m http.server 8080 --directory dist
```

Abra `http://localhost:8080` no Chromium. Não abra o HTML diretamente pelo gerenciador de arquivos: módulos JavaScript e o service worker precisam de um servidor HTTP. Os dados da biblioteca são locais; conexão é necessária para login CREDE, EmailJS e Google Drive.

## Aplicativo desktop Tauri

O desktop usa Tauri v2 e mantém a mesma interface web. A versão atual é 1.0.7. Para desenvolvimento, instale as dependências Linux do Tauri e execute `npm run tauri dev`. Para gerar o pacote Debian da versão atual, execute `npm run build:desktop`; o arquivo será criado em `src-tauri/target/release/bundle/deb/`.

No Ubuntu, o build requer Rust/Cargo e os pacotes `build-essential`, `curl`, `wget`, `file`, `libssl-dev`, `libxdo-dev`, `librsvg2-dev` e `libwebkit2gtk-4.1-dev`. O login CREDE, a autorização do Drive e a câmera devem ser validados no WebView do sistema após a instalação.

Para ler um QR code, clique em **Escanear QR code** e depois em **Câmera deste aparelho**. Quando solicitado, permita o acesso. Se já tiver bloqueado: no navegador, abra o cadeado ao lado do endereço e altere a permissão de câmera; no Linux, vá a **Configurações do sistema > Privacidade > Câmera** e habilite o Libris. Feche e reabra o leitor. Também é possível usar um leitor externo USB/Bluetooth ou digitar o código do livro.

## Instalar no Chromium/Xubuntu

1. Abra a URL HTTPS publicada ou `http://localhost:8080` no Chromium.
2. Use o ícone **Instalar Libris** na barra de endereço ou abra o menu do Chromium e escolha **Instalar Libris**.
3. Confirme a instalação. O Libris será aberto em uma janela própria e aparecerá no lançador de aplicativos.

Se a opção de instalação não aparecer, confira se o manifest, os ícones e o service worker estão acessíveis na mesma origem. Para instalação em outro computador ou celular, a origem precisa usar HTTPS; a exceção para HTTP vale somente para `localhost`.

## Publicar

Para publicar o shell estático fora do Firebase, use Cloudflare Pages conectado ao repositório:

1. Crie um projeto Pages e conecte este repositório.
2. Configure o comando de build como `npm run build:hosting` e o diretório de saída como `dist`.
3. Aguarde a URL HTTPS e teste o login e as funções do app antes de compartilhar o endereço.

Cadastre o domínio HTTPS definitivo e configure a tela de consentimento OAuth para o fluxo de backup no Google Drive. Nunca coloque Client Secret, chave privada ou token de serviço no frontend.

O build copia doze arquivos necessários ao app: HTML, CSS, JavaScript, módulos de armazenamento local, configuração de autenticação, imagem de marca, manifest, service worker e dois ícones. Scripts de migração, funções de servidor, chaves privadas e arquivos de desenvolvimento não são publicados.

## Limites atuais

- A autenticação CREDE ainda depende do Firebase Authentication por email/senha; o provedor Google é usado somente para autorização do Drive. Não há Client Secret no frontend.
- A migração de dados existentes requer o arquivo de backup legado. O app não conecta ao Firestore.
- O backup Drive não roda com o navegador fechado. Verifique o status semanalmente no computador da biblioteca.
- A validação automatizada cobre armazenamento e CSV. O login administrativo e o upload real ainda precisam ser testados com a configuração Firebase e a Drive API do ambiente de produção.
