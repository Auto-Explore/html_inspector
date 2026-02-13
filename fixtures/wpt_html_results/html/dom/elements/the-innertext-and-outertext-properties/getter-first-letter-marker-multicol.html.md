# html/dom/elements/the-innertext-and-outertext-properties/getter-first-letter-marker-multicol.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/dom/elements/the-innertext-and-outertext-properties/getter-first-letter-marker-multicol.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Test innerText/outerText for a combination of a list item with ::first-letter in multicol</title>
<link rel="help" href="https://html.spec.whatwg.org/multipage/dom.html#dom-innertext">
<link rel="help" href="https://crbug.com/1174985">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<style>
  #item { display: list-item; }
  #item::first-letter { background: lime; }
  .col { column-count: 1; }
</style>
<div id="item" class="col"><div class="col">PASS</div></div>
<script>
  test(() => {
    assert_equals(item.innerText, "PASS", "innerText");
    assert_equals(item.outerText, "PASS", "outerText");
  }, "");
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
  "source_name": "html/dom/elements/the-innertext-and-outertext-properties/getter-first-letter-marker-multicol.html"
}
```
