<!-- git diff | rg '^[^\s]\s*//' | sort -->
## 0.1.53 (2024-08-16)
- Improve lookup speed
- mime-db 1.53.0

### New extensions
- `application/appinstaller` → `appinstaller`
- `application/appx` → `appx`
- `application/appxbundle` → `appxbundle`
- `application/automationml-aml+xml` → `aml`
- `application/automationml-amlx+zip` → `amlx`
- `application/cwl` → `cwl`
- `application/fdf` → `fdf`
- `application/msixbundle` → `msixbundle`
- `application/msix` → `msix`
- `application/prs.xsf+xml` → `xsf`
- `application/sql` → `sql`
- `application/vnd.geogebra.slides` → `ggs`
- `application/vnd.gov.sk.xmldatacontainer+xml` → `xdcf`
- `application/vnd.nato.bindingdataobject+xml` → `bdo`
- `application/vnd.pwg-xhtml-print+xml` → `xhtm`
- `application/xfdf` → `xfdf`
- `audio/aac` → `adts`
- `image/dpx` → `dpx`
- `image/jxl` → `jxl`
- `model/jt` → `jt`
- `model/prc` → `prc`
- `model/u3d` → `u3d`
- `model/vnd.bary` → `bary`
- `model/vnd.cld` → `cld`
- `model/vnd.pytha.pyox` → `pyo`
- `model/vnd.usda` → `usda`
- `text/javascript` → `js`
- `text/wgsl` → `wgsl`

### Changed extensions
- `application/ecmascript`: `es` → `ecma`
- `application/mp4`: `mp4s` → `mp4`
- `application/pgp-signature`: `asc` → `sig`
- `text/markdown`: `markdown` → `md`

## 0.1.52 (2022-02-21)
- Make generated code more compact
- mime-db 1.52.0

### New extensions
- `application/cpl+xml` → `cpl`
- `application/dash-patch+xml` → `mpp`
- `application/media-policy-dataset+xml` → `mpf`
- `application/pgp-keys` → `asc`
- `application/watcherinfo+xml` → `wif`
- `image/avci` → `avci`
- `image/avcs` → `avcs`

## 0.1.51 (2021-11-09)
- mime-db 1.51.0

### New extensions
- `application/vnd.age` → `age`
- `text/vnd.familysearch.gedcom` → `ged`

## 0.1.50 (2021-09-16)
- mime-db 1.50.0

### New extensions
- `application/express` → `exp`
- `application/x-iwork-keynote-sffkey` → `key`
- `application/x-iwork-numbers-sffnumbers` → `numbers`
- `application/x-iwork-pages-sffpages` → `pages`
- `model/step+xml` → `stpx`

## 0.1.49 (2021-08-01)
- Lower MSRV to 1.6
- Start tracking mime-db version number
- mime-db 1.49.0

### New extensions
- `application/trig` → `trig`
- `model/step+zip` → `stpz`
- `model/step-xml+zip` → `stpxz`

## 0.1.4 (2021-05-31)
- Remove build dependencies
- mime-db 1.48.0

### New extensions
- `application/vnd.mapbox-vector-tile` → `mvt`

## 0.1.3 (2021-04-01)
- mime-db 1.47.0

### New extensions
- `model/vnd.sap.vds` → `vds`

### Changed extensions
- `application/ecmascript`: `ecma` → `es`

### Removed extensions
- `application/mrb-consumer+xml` (`xdf`)
- `application/mrb-publish+xml` (`xdf`)
- `application/xcap-error+xml` (`xer`)

## 0.1.2 (2021-03-03)
- Do not rely on unstable `Debug` output in build script

## 0.1.1 (2021-02-15)
- mime-db 1.46.0

### New extensions
- `audio/amr` → `amr`
- `video/iso.segment` → `m4s`

## 0.1.0 (2021-02-12)
- Initial release
- mime-db 1.45.0
