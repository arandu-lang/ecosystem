# Benchmark Real-World: miniz Transpilation

Este diretório contém os fontes da biblioteca **miniz** (implementação C pura de zlib/deflate/inflate/zip) e os artefatos de transpilação para Arandu gerados pelo `c2arandu`.

## Arquivos

- `miniz.h` / `miniz.c`: Cabeçalho e implementação de alto nível do miniz.
- `miniz_common.h`, `miniz_tdef.h`, `miniz_tinfl.h`, `miniz_zip.h`: Módulos de compressão, descompressão e manipulação ZIP.
- `miniz_h.aru`: Módulo gerado automaticamente pelo `c2arandu` contendo 8 structs, 18 enums/constantes, 46 typedefs e 134 funções `extern "C"`. Totalmente validado pelo compilador (`arandu check`).

## Paridade e Verificação com JIT

Os componentes fundamentais de integridade e checksum do `miniz` foram transpilados e verificados com paridade exata de execução no compilador Arandu JIT (Cranelift):

1. **Adler-32 (`mz_adler32`)**:
   - Calculado sobre payload `"World"` com seed 1.
   - Resultado: `101057033` (exatamente idêntico ao C miniz e Python zlib).
   - Teste automatizado: `tests/miniz_parity.rs::test_miniz_adler32_parity_jit`.

2. **CRC-32 (`mz_crc32`)**:
   - Algoritmo compacto CCITT CRC-32 (Karl Malbrain).
   - Calculado sobre payload `"Hello World"` com seed 0.
   - Resultado: `1243066710` (padrão IEEE 802.3 / zlib).
   - Teste automatizado: `tests/miniz_parity.rs::test_miniz_crc32_parity_jit`.

3. **Validação Estática e Typechecking**:
   - Todas as structs (`MzStream`, `InflateState`, `MzZipArchive`, etc.) e enums/constantes (`MZ_OK`, `MZ_STREAM_END`, etc.) compilam no typechecker oficial do Arandu.
   - Teste automatizado: `tests/miniz_parity.rs::test_miniz_header_transpile_and_check`.
