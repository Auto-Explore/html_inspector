# html/syntax/speculative-parsing/generated/document-write/meta-viewport-link-stylesheet-media.tentative.sub.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/syntax/speculative-parsing/generated/document-write/meta-viewport-link-stylesheet-media.tentative.sub.html",
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
<title>Speculative parsing, document.write(): meta-viewport-link-stylesheet-media</title>
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<script src=/common/utils.js></script>
<script src=/html/syntax/speculative-parsing/resources/speculative-parsing-util.js></script>
<script>
  setup({single_test: true});
  const uuid = token();
  expect_fetched_onload(uuid, false)
    .then(compare_with_nonspeculative(uuid, 'meta-viewport-link-stylesheet-media', true))
    .then(done);
  document.write(`
    <script src="/common/slow.py?delay=1500"><\/script>
    <script>
     document.write('<plaintext>');
    <\/script>
    <\!-- speculative case in document.write -->
    <meta name=viewport content="width=400, initial-scale=1"><link rel=stylesheet href="/html/syntax/speculative-parsing/resources/stash.py?action=put&amp;uuid=${uuid}&amp;encodingcheck=&Gbreve;" media="(min-width: 401px)">
  `);
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
  "source_name": "html/syntax/speculative-parsing/generated/document-write/meta-viewport-link-stylesheet-media.tentative.sub.html"
}
```
