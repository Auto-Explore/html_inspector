# html/semantics/selectors/pseudo-classes/inrange-outofrange-type-change.html

Counts:
- errors: 0
- warnings: 5
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/selectors/pseudo-classes/inrange-outofrange-type-change.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>Selector: pseudo-classes (:in-range, :out-of-range) input type change</title>
<link rel="author" title="Rune Lillesveen" href="mailto:rune@opera.com">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#pseudo-classes">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<style>
  span {
    color: red;
  }
  #t1:in-range + span {
    color: green;
  }
  #t2:out-of-range + span {
    color: green;
  }
</style>
<input id="t1" type="text" min="0" max="10" value="5">
<span id="sibling1">This text should be green.</span>
<input id="t2" type="text" min="0" max="10" value="50">
<span id="sibling2">This text should be green.</span>
<script>
  test(() => {
    assert_equals(getComputedStyle(sibling1).color, "rgb(255, 0, 0)",
      "Not matching :in-range for type=text");

    t1.type = "number";

    assert_equals(getComputedStyle(sibling1).color, "rgb(0, 128, 0)",
      "Matching :in-range for type=number");
  }, "Evaluation of :in-range changes for input type change.");

  test(() => {
    assert_equals(getComputedStyle(sibling2).color, "rgb(255, 0, 0)",
      "Not matching :out-of-range for type=text");

    t2.type = "number";

    assert_equals(getComputedStyle(sibling2).color, "rgb(0, 128, 0)",
      "Matching :in-range for type=number");
  }, "Evaluation of :out-of-range changes for input type change.");
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.input.min.disallowed_for_type",
      "message": "Attribute “min” not allowed on element “input” at this point.",
      "severity": "Warning",
      "span": {
        "byte_end": 579,
        "byte_start": 525,
        "col": 1,
        "line": 19
      }
    },
    {
      "category": "Html",
      "code": "html.input.max.disallowed_for_type",
      "message": "Attribute “max” not allowed on element “input” at this point.",
      "severity": "Warning",
      "span": {
        "byte_end": 579,
        "byte_start": 525,
        "col": 1,
        "line": 19
      }
    },
    {
      "category": "Html",
      "code": "html.input.min.disallowed_for_type",
      "message": "Attribute “min” not allowed on element “input” at this point.",
      "severity": "Warning",
      "span": {
        "byte_end": 689,
        "byte_start": 634,
        "col": 1,
        "line": 21
      }
    },
    {
      "category": "Html",
      "code": "html.input.max.disallowed_for_type",
      "message": "Attribute “max” not allowed on element “input” at this point.",
      "severity": "Warning",
      "span": {
        "byte_end": 689,
        "byte_start": 634,
        "col": 1,
        "line": 21
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
  "source_name": "html/semantics/selectors/pseudo-classes/inrange-outofrange-type-change.html"
}
```
