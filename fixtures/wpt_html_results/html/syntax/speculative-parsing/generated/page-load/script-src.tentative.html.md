# html/syntax/speculative-parsing/generated/page-load/script-src.tentative.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/syntax/speculative-parsing/generated/page-load/script-src.tentative.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<!-- DO NOT EDIT. This file has been generated. Source:
     /html/syntax/speculative-parsing/tools/generate.py
-->
<meta charset=utf-8>
<title>Speculative parsing, page load: script-src</title>
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<script src=/common/utils.js></script>
<script src=/html/syntax/speculative-parsing/resources/speculative-parsing-util.js></script>
<body>
<script>
  setup({single_test: true});
  const uuid = token();
  const iframe = document.createElement('iframe');
  iframe.src = `resources/script-src-framed.sub.html?uuid=${uuid}`;
  document.body.appendChild(iframe);
  expect_fetched_onload(uuid, true)
    .then(compare_with_nonspeculative(uuid, 'script-src', true))
    .then(done);
</script>
```

```json
{
  "messages": [
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/syntax/speculative-parsing/generated/page-load/script-src.tentative.html"
}
```
