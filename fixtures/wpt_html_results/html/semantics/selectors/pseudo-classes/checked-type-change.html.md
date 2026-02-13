# html/semantics/selectors/pseudo-classes/checked-type-change.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/selectors/pseudo-classes/checked-type-change.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>Selector: pseudo-class :checked input type change</title>
<link rel="author" title="Rune Lillesveen" href="mailto:rune@opera.com">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#pseudo-classes">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<style>
  span { color: red }
  :checked + span { color: green }
</style>
<input id="checked" type="text" checked>
<span id="sibling">This text should be green.</span>
<script>
  test(() => {
    assert_equals(getComputedStyle(sibling).color, "rgb(255, 0, 0)",
      "Not matching :checked for type=text");

    checked.type = "radio";

    assert_equals(getComputedStyle(sibling).color, "rgb(0, 128, 0)",
      "Matching :checked for type=radio");
  }, "Evaluation of :checked changes on input type change.");
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.input.checked.disallowed_for_type",
      "message": "Attribute “checked” not allowed on element “input” at this point.",
      "severity": "Warning",
      "span": {
        "byte_end": 477,
        "byte_start": 437,
        "col": 1,
        "line": 12
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
  "source_name": "html/semantics/selectors/pseudo-classes/checked-type-change.html"
}
```
