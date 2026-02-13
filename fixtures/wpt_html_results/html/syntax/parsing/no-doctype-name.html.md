# html/syntax/parsing/no-doctype-name.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/syntax/parsing/no-doctype-name.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype>
<meta charset=utf-8>
<title>Doctype without root name should have empty-string name in the DOM even if null in the tokenizer spec.</title>
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<script>
setup({explicit_done:true});
window.onload = function() {
  test(function () {
    assert_equals(document.doctype.name, "", "Top-level");
    let iframes = document.getElementsByTagName("iframe");
    for (let i = 0; i < iframes.length; ++i) {
      let iframe = iframes[i];
      assert_equals(iframe.contentDocument.doctype.name, "", iframe.title);
    }
  }, "Doctype without root name should have the empty string as the name.");
  done();
}
</script>
<iframe src="support/no-doctype-name-space.html" title='space'></iframe>
<iframe src="support/no-doctype-name-line.html" title='line'></iframe>
<iframe src="support/no-doctype-name-eof.html" title='eof'></iframe>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.tokenizer.doctype.missing_space_before_name",
      "message": "Missing space before doctype name.",
      "severity": "Warning",
      "span": {
        "byte_end": 2,
        "byte_start": 0,
        "col": 1,
        "line": 1
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
  "source_name": "html/syntax/parsing/no-doctype-name.html"
}
```
