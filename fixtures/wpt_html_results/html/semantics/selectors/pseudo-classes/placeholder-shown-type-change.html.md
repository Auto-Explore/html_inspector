# html/semantics/selectors/pseudo-classes/placeholder-shown-type-change.html

Counts:
- errors: 1
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/selectors/pseudo-classes/placeholder-shown-type-change.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>Selector: pseudo-class :placeholder-shown input type change</title>
<link rel="author" title="Rune Lillesveen" href="mailto:rune@opera.com">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#pseudo-classes">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<style>
  span {
    color: red;
  }
  :placeholder-shown + span {
    color: green;
  }
</style>
<input id="input" type="submit" placeholder="placeholder"></input>
<span id="sibling">This text should be green.</span>
<script>
  test(() => {
    assert_equals(getComputedStyle(sibling).color, "rgb(255, 0, 0)",
      "Not matching :placeholder-shown for type=submit");

    input.type = "text";
    assert_equals(getComputedStyle(sibling).color, "rgb(0, 128, 0)",
      "Matching :placeholder-shown for type=text");
  }, "Evaluation of :placeholder-shown changes for input type change.");
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “input”.",
      "severity": "Error",
      "span": {
        "byte_end": 537,
        "byte_start": 529,
        "col": 59,
        "line": 16
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
  "source_name": "html/semantics/selectors/pseudo-classes/placeholder-shown-type-change.html"
}
```
