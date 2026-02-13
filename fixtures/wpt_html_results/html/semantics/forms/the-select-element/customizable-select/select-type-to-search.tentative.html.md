# html/semantics/forms/the-select-element/customizable-select/select-type-to-search.tentative.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-select-element/customizable-select/select-type-to-search.tentative.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta name="timeout" content="long">
<link rel="help" href="http://crbug.com/362922565">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-vendor.js"></script>

<!-- This test is optional because the HTML spec does not require that specific
  behaviors are mapped to specific keyboard buttons. -->

<style>
  select, select::picker(select) {
    appearance: base-select;
  }
</style>

<select id="target">
  <option></option>
  <option>alpha</option>
  <option id="bravo">bravo</option>
  <option>charlie</option>
</select>

<script>
  const Enter = '\uE007';
  const Space = ' ';

  promise_test(async (t) => {
    assert_false(
      target.matches(':open'),
      'The select should initially be closed.'
    );

    assert_equals(
      target.value,
      '',
      'Initial select value should be empty.'
    );

    target.focus();
    assert_equals(
      document.activeElement,
      target,
      'The select should be focused.'
    );

    await test_driver.send_keys(document.activeElement, Space);
    assert_true(
      target.matches(':open'),
      'The select should be open after pressing space.'
    );

    await test_driver.send_keys(document.activeElement, 'b');
    assert_equals(
      document.activeElement,
      bravo,
      'The `bravo` option should be focused after pressing `b`.'
    );
    assert_equals(
      target.value,
      '',
      'The select value should still be empty since the option was not explicitly selected.'
    );

    await test_driver.send_keys(document.activeElement, Enter);
    assert_equals(
      target.value,
      'bravo',
      'The select value should be `bravo` after explicit selection.'
    );
  }, 'Type to search should focus but not select an option until selection is confirmed.');
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.option.empty_without_label",
      "message": "Element “option” without attribute “label” must not be empty.",
      "severity": "Warning",
      "span": {
        "byte_end": 581,
        "byte_start": 572,
        "col": 11,
        "line": 19
      }
    },
    {
      "category": "Html",
      "code": "html.head.title.missing",
      "message": "Element “head” is missing a required instance of child element “title”.",
      "severity": "Warning",
      "span": null
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/semantics/forms/the-select-element/customizable-select/select-type-to-search.tentative.html"
}
```
