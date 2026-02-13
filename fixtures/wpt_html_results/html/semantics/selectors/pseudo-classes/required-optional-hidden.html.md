# html/semantics/selectors/pseudo-classes/required-optional-hidden.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/selectors/pseudo-classes/required-optional-hidden.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>Selector: pseudo-classes (:required, :optional) for hidden input</title>
<link rel="author" title="Rune Lillesveen" href="mailto:rune@opera.com">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#pseudo-classes">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<style>
  span {
    color: red;
    background-color: pink;
  }
  :required + span {
    color: green;
  }
  :not(:optional) + span {
    background-color: lime;
  }
</style>
<input id="hiddenInput" type="hidden" required>
<span id="sibling">This text should be green on lime background.</span>
<script>
  test(() => {
    assert_equals(getComputedStyle(sibling).color, "rgb(255, 0, 0)",
      "Not matching :required for type=hidden");
    assert_equals(getComputedStyle(sibling).backgroundColor, "rgb(255, 192, 203)",
      "Matching :optional for type=hidden");

    hiddenInput.type = "text";

    assert_equals(getComputedStyle(sibling).color, "rgb(0, 128, 0)",
      "Matching :required for type=text");
    assert_equals(getComputedStyle(sibling).backgroundColor, "rgb(0, 255, 0)",
      "Matching :not(:optional) for type=text");
  }, "Evaluation of :required and :optional changes for input type change.");
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.input.required.disallowed_for_type",
      "message": "Attribute “required” not allowed on element “input” at this point.",
      "severity": "Warning",
      "span": {
        "byte_end": 601,
        "byte_start": 554,
        "col": 1,
        "line": 20
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
  "source_name": "html/semantics/selectors/pseudo-classes/required-optional-hidden.html"
}
```
