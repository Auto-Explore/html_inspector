# html/semantics/forms/the-input-element/input-stepdown-weekmonth.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-input-element/input-stepdown-weekmonth.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE HTML>
<html>
<title>Forms</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<h3>input_stepDown</h3>
<input type="month" id="month_input" min="2011-02" step="1" value="2010-02">
<input type="week" id="week_input" min="2011-W02" step="1" value="2010-W02">

<script>
   function testStepDownOverflow(id, value, type) {
      test(function() {
        var input = document.getElementById(id);
        input.stepDown();
        assert_equals(input.value, value, "value shouldn't change.");
      }, "Calling stepDown() on input - " + type + " - where value < min should not modify value.");
    }

    testStepDownOverflow("month_input", "2010-02", "month");
    testStepDownOverflow("week_input", "2010-W02", "week");
</script>
</html>
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
  "source_name": "html/semantics/forms/the-input-element/input-stepdown-weekmonth.html"
}
```
