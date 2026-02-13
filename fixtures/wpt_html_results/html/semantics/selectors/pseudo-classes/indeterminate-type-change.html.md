# html/semantics/selectors/pseudo-classes/indeterminate-type-change.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/selectors/pseudo-classes/indeterminate-type-change.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>Selector: pseudo-class :indeterminate input type change</title>
<link rel="author" title="Rune Lillesveen" href="mailto:rune@opera.com">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#pseudo-classes">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<style>
  span { color: red }
  :indeterminate + span { color: green }
</style>
<input id="indeterminate" type="text">
<span id="sibling">This text should be green.</span>
<script>
  test(() => {
    assert_equals(getComputedStyle(sibling).color, "rgb(255, 0, 0)",
      "Not matching :indeterminate for type=text");

    indeterminate.type = "radio";

    assert_equals(getComputedStyle(sibling).color, "rgb(0, 128, 0)",
      "Matching :indeterminate for type=radio");
  }, "Evaluation of :indeterminate changes on input type change.");
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
  "source_name": "html/semantics/selectors/pseudo-classes/indeterminate-type-change.html"
}
```
