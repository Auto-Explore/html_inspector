# html/semantics/forms/the-datalist-element/datalistoptions.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-datalist-element/datalistoptions.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset=utf-8>
<title>Datalist element options</title>
<link rel="author" title="Denis Ah-Kang" href="mailto:denis@w3.org">
<link rel=help href="https://html.spec.whatwg.org/multipage/#the-datalist-element">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id="log"></div>
<label>
  Number:
  <input list=numbers>
</label>
<datalist id=numbers>
  <label> Select number:
    <select id=num>
      <option label="zero" value="0">
      <option label="one" value="1">
      <option label="two">2
      <option label="three" disabled>3
      <option>
    </select>
  </label>
</datalist>
<script>
  test(function(){
    var datalist = document.getElementById('numbers'),
        labels = [],
        values = [];
    assert_equals(datalist.options.length, 5);

    for (var i = 0, len = datalist.options.length; i < len; i++) {
      assert_equals(datalist.options[i], datalist.options.item(i));
      labels.push(datalist.options[i].label);
      values.push(datalist.options[i].value);
    }
    assert_array_equals(labels, ["zero", "one", "two", "three", ""]);
    assert_array_equals(values, ["0", "1", "2", "3", ""]);
  }, "options label/value");

  test(function(){
    assert_false(document.getElementById('num').willValidate);
  }, "If an element has a datalist element ancestor, it is barred from constraint validation");
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
  "source_name": "html/semantics/forms/the-datalist-element/datalistoptions.html"
}
```
