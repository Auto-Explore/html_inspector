# html/semantics/forms/the-select-element/select-add.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-select-element/select-add.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>HTMLSelectElement Test: add()</title>
<link rel="author" title="Intel" href="http://www.intel.com/">
<link rel="help" href="https://html.spec.whatwg.org/multipage/form-elements.html#dom-select-add-dev">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<form style="display:none">
  <option id="testoption">
    <select id="testselect1">
    </select>
    <select id="testselect2">
      <option>TEST</option>
    </select>
  </option>
</form>

<script>

test(() => {
  let testselect1 = document.getElementById("testselect1");
  let opt1 = new Option("Marry","1");
  testselect1.add(opt1);
  assert_equals(testselect1.options[0].value, "1");
}, "test that HTMLSelectElement.add method can add option element");

test(() => {
  let testselect2 = document.getElementById("testselect2");
  let opt2 = document.getElementById("testoption");
  assert_throws_dom("HierarchyRequestError", () => {
    testselect2.add(opt2);
  });
}, "test that HierarchyRequestError exception must be thrown when element is an ancestor of the element into which it is to be inserted");

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
        "byte_end": 538,
        "byte_start": 529,
        "col": 3,
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
  "source_name": "html/semantics/forms/the-select-element/select-add.html"
}
```
