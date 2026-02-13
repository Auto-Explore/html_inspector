# html/rendering/non-replaced-elements/lists/li-text-align.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/lists/li-text-align.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>text-align: match-parent on li</title>
<meta name="viewport" content="width=device-width">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<ul dir=rtl><li dir=ltr></li></ul>
<ul dir=ltr><li dir=rtl></li></ul>
<script>
  test(() => {
    const li = document.querySelector('li[dir=ltr]');
    assert_equals(getComputedStyle(li).textAlign, 'right');
  }, '<ul dir=rtl><li dir=ltr>');

  test(() => {
    const li = document.querySelector('li[dir=rtl]');
    assert_equals(getComputedStyle(li).textAlign, 'left');
  }, '<ul dir=ltr><li dir=rtl>');
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
  "source_name": "html/rendering/non-replaced-elements/lists/li-text-align.html"
}
```
