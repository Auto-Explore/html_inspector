# html/rendering/non-replaced-elements/the-page/iframe-body-margin-attributes.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/the-page/iframe-body-margin-attributes.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>iframe and body margin attributes</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<body marginwidth=20 marginheight=20 topmargin=10 rightmargin=10 bottommargin=10 leftmargin=10>
<iframe data-desc="iframe marginwidth vs child body leftmargin" src="support/body-topmargin-leftmargin.html" marginwidth=10 marginheight=10></iframe>
<iframe data-desc="iframe marginwidth vs child body marginwidth" src="support/body-marginwidth-marginheight.html" marginwidth=10 marginheight=10></iframe>
<script>
setup({explicit_done: true});

onload = () => {
  test(() => {
    const style = getComputedStyle(document.body);
    assert_style_props(style);
  }, 'body marginwidth vs body leftmargin');

  [].forEach.call(document.querySelectorAll('iframe'), iframe => {
    test(() => {
      const win = iframe.contentWindow;
      const style = win.getComputedStyle(win.document.body);
      assert_style_props(style);
    }, iframe.dataset.desc);
  });
  done();
}

function assert_style_props(style) {
  for (let prop of ['marginTop', 'marginRight', 'marginBottom', 'marginLeft']) {
    assert_equals(style[prop], '20px', prop);
  }
}
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
  "source_name": "html/rendering/non-replaced-elements/the-page/iframe-body-margin-attributes.html"
}
```
