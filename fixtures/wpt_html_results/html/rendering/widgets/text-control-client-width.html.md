# html/rendering/widgets/text-control-client-width.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/widgets/text-control-client-width.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>text control with `display: inline` must not have 0 client width</title>
<link rel="help" href="https://html.spec.whatwg.org/multipage/rendering.html#form-controls">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<input id="input" style="display: inline;">
<textarea id="textarea" style="display: inline;"></textarea>

<script>
test(() => {
  assert_greater_than(document.querySelector("#input").clientWidth, 0);
}, "Input with `display: inline` should have positive client width");

test(() => {
  assert_greater_than(document.querySelector("#textarea").clientWidth, 0);
}, "Textarea with `display: inline` should have positive client width");
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
  "source_name": "html/rendering/widgets/text-control-client-width.html"
}
```
