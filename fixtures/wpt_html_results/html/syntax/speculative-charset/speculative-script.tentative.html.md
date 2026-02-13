# html/syntax/speculative-charset/speculative-script.tentative.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/syntax/speculative-charset/speculative-script.tentative.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<head>
<meta charset="windows-1253">
<title>Speculative script</title>
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<script src=/common/utils.js></script>
<body>
<script>
const uuid = token();

window.onmessage = function(e) {
  // The script is first speculatively loaded in the windows-1253 context (inherited from this doc), so
  // the Greek hex NCR turns into a byte. This byte is stored. Then the script is fetched
  // non-speculatively, because the Greek hexNCR in the URL makes the URL not match in the
  // windows-1251 context. Now the Greek hex NCR turns in to a decimal NCR and the original Greek
  // character comes back as a byte that gets a Cyrillic interpretation.
  assert_equals(e.data, `token: ${uuid}, character: &#950;, previous character: \u0436, byte: \u0436`, "Check result");
  done();
}

setup({single_test: true});
const iframe = document.createElement('iframe');
iframe.src = `support/speculative-script.py?uuid=${uuid}`;
document.body.appendChild(iframe);
</script>
```

```json
{
  "messages": [
    {
      "category": "I18n",
      "code": "i18n.meta.charset.mismatch",
      "message": "Internal encoding declaration “windows-1253” disagrees with the actual encoding of the document (“utf-8”).",
      "severity": "Warning",
      "span": {
        "byte_end": 59,
        "byte_start": 30,
        "col": 1,
        "line": 4
      }
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/syntax/speculative-charset/speculative-script.tentative.html"
}
```
