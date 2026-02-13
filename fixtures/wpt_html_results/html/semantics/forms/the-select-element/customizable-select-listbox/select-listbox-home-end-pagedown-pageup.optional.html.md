# html/semantics/forms/the-select-element/customizable-select-listbox/select-listbox-home-end-pagedown-pageup.optional.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-select-element/customizable-select-listbox/select-listbox-home-end-pagedown-pageup.optional.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta name=timeout content=long>
<link rel=author href="mailto:jarhar@chromium.org">
<link rel=help href="https://issues.chromium.org/issues/450483229">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-vendor.js"></script>
<script src="/resources/testdriver-actions.js"></script>

<!-- This test is optional because the HTML spec does not require that specific
  behaviors are mapped to specific keyboard buttons. -->

<style>
select {
  appearance: base-select;
}
</style>

<select id=single size=3></select>
<select id=multiple multiple size=3></select>

<script>
const homeKey = '\uE011';
const endKey = '\uE010';
const pageUpKey = '\uE00E';
const pageDownKey = '\uE00F';

function pressKey(key) {
  return (new test_driver.Actions()
    .keyDown(key)
    .keyUp(key))
    .send();
}

const numOptions = 50;
const size = 3;

['single', 'multiple'].forEach(id => {
  const select = document.getElementById(id);
  select.innerHTML = '';
  const options = [];
  for (let i = 0; i < numOptions; i++) {
    const option = document.createElement('option');
    option.textContent = i;
    options.push(option);
    select.appendChild(option);
  }

  const firstTop = options[0].getBoundingClientRect().top;
  const lastTop = options[2].getBoundingClientRect().top;

  promise_test(async () => {
    options[0].focus();
    assert_equals(document.activeElement, options[0],
      'The first option should be focused at the start of the test.');

    await pressKey(endKey);
    assert_equals(document.activeElement, options[numOptions - 1],
      'End should focus the last option.');
    assert_equals(document.activeElement.getBoundingClientRect().top, lastTop,
      'End should scroll the last option into view.');

    await pressKey(homeKey);
    assert_equals(document.activeElement, options[0],
      'Home should focus the first option.');
    assert_equals(document.activeElement.getBoundingClientRect().top, firstTop,
      'Home should scroll the first option into view.');

    await pressKey(pageDownKey);
    assert_equals(document.activeElement, options[size - 1],
      'PageDown should focus the last visible option.');
    assert_equals(document.activeElement.getBoundingClientRect().top, lastTop,
      'PageDown should not scroll if the option is already visible.');

    await pressKey(pageDownKey);
    assert_equals(document.activeElement, options[(size - 1) * 2],
      'Second PageDown should focus the next last visible option.');
    assert_equals(document.activeElement.getBoundingClientRect().top, lastTop,
      'Second PageDown should scroll the next last visible option into view.');

    await pressKey(pageUpKey);
    assert_equals(document.activeElement, options[size - 1],
      'PageUp should focus the first visible option.');
    assert_equals(document.activeElement.getBoundingClientRect().top, firstTop,
      'PageUp should not scroll if the option is already visible.');

    await pressKey(pageUpKey);
    assert_equals(document.activeElement, options[0],
      'Second PageUp should focus the next first visible option.');
    assert_equals(document.activeElement.getBoundingClientRect().top, firstTop,
      'Second pageUp should scroll the next first visible option into view.');
  }, `${id}: Home/End/PageUp/PageDown should move focus between options and scroll.`);
});
</script>
```

```json
{
  "messages": [
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
  "source_name": "html/semantics/forms/the-select-element/customizable-select-listbox/select-listbox-home-end-pagedown-pageup.optional.html"
}
```
