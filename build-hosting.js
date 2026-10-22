import('vite').then(({ build }) => build()).catch((error) => {
  console.error('Falha ao gerar o build do Libris:', error)
  process.exitCode = 1
})
