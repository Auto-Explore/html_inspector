# html/semantics/forms/the-input-element/input-stepdown-02.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-input-element/input-stepdown-02.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE HTML>
<title>Input Step Down</title>

<link rel="help" href="https://html.spec.whatwg.org/multipage/input.html#dom-input-stepup">

<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<input type='number' id='input'>

<script>
  const input = document.getElementById("input");

  function testStepDown(initialValue, minValue, expectedValue) {
    input.value = initialValue;
    input.min = minValue;

    input.stepDown();

    assert_equals(input.value, expectedValue);
  }

  const tests = [
    { initialValue: '', minValue: '', expectedValue: '-1', description: 'stepDown() on input with no initial or min values' },
    { initialValue: '', minValue: '7', expectedValue: '7', description: 'stepDown() on input with no initial value and positive min value' },
    { initialValue: '', minValue: '-7', expectedValue: '-1', description: 'stepDown() on input with no initial value and negative min value' },
    { initialValue: '7', minValue: '7', expectedValue: '7', description: 'stepDown() on input with initial value equal to min value' },
    { initialValue: '3', minValue: '7', expectedValue: '3', description: 'stepDown() on input with initial value less than min value' },
    { initialValue: '10', minValue: '7', expectedValue: '9', description: 'stepDown() on input with initial value greater than min value' },
  ];

  for(const t of tests) {
    test(()=>{
      testStepDown(
        t.initialValue,
        t.minValue,
        t.expectedValue
      );
    },
    t.description);
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
  "source_name": "html/semantics/forms/the-input-element/input-stepdown-02.html"
}
```
