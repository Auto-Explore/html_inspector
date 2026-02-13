# html/semantics/forms/the-select-element/customizable-select/selectedcontentelement-attr.tentative.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-select-element/customizable-select/selectedcontentelement-attr.tentative.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel=author href="mailto:jarhar@chromium.org">
<link rel=help href="https://github.com/openui/open-ui/issues/1063">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<select style="appearance:base-select">
  <option class=one value=one>one<span>span</span></option>
  <option class=two value=two>two<span>span</span></option>
</select>
<selectedcontent id=myselectedcontent></selectedcontent>

<script>
promise_test(async () => {
  const select = document.querySelector('select');
  const selectedcontent = document.querySelector('selectedcontent');
  const optionOne = document.querySelector('option.one');
  const optionTwo = document.querySelector('option.two');

  select.setAttribute('selectedcontentelement', 'myselectedcontent');
  assert_equals(selectedcontent.innerHTML, optionOne.innerHTML,
    'Setting the selectedcontentelement attribute via setAttribute should synchronously assign the contents of <selectedcontent>.');

  select.removeAttribute('selectedcontentelement');
  assert_equals(selectedcontent.innerHTML, '',
    'Removing the selectedcontentelement attribute via removeAttribute should synchronously clear the contents of the <selectedcontent>.');

  select.selectedContentElement = selectedcontent;
  assert_equals(selectedcontent.innerHTML, optionOne.innerHTML,
    'Setting the selectedcontentelement attribute via IDL should synchronously assign the contents of <selectedcontent>.');

  select.selectedContentElement = null;
  assert_equals(selectedcontent.innerHTML, '',
    'Removing the selectedcontentelement attribute via IDL should synchronously clear the contents of the <selectedcontent>.');

  select.selectedContentElement = selectedcontent;
  assert_equals(selectedcontent.innerHTML, optionOne.innerHTML,
    'Re-setting the selectedcontentelement attribute via IDL should synchronously assign the contents of <selectedcontent>.');

  let oldInnerHTML = optionOne.innerHTML;
  optionOne.querySelector('span').remove();
  await new Promise(queueMicrotask);
  assert_equals(selectedcontent.innerHTML, oldInnerHTML,
    'Mutating the selected <option> should not update the <selectedcontent> contents.');

  select.value = 'two';
  assert_equals(selectedcontent.innerHTML, optionTwo.innerHTML,
    'Changing which <option> is selected should synchronously update the <selectedcontent> contents.');
}, 'The selectedcontentelement attribute should set up an association between a select and a selectedcontent.');
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
  "source_name": "html/semantics/forms/the-select-element/customizable-select/selectedcontentelement-attr.tentative.html"
}
```
