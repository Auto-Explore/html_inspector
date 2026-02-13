# html/semantics/selectors/pseudo-classes/required-optional.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/selectors/pseudo-classes/required-optional.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset=utf-8>
<title>Selector: pseudo-classes (:required, :optional)</title>
<link rel="author" title="Denis Ah-Kang" href="mailto:denis@w3.org" id=link1>
<link rel=help href="https://html.spec.whatwg.org/multipage/#pseudo-classes" id=link2>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="utils.js"></script>
<div id="log"></div>
<input type=text id=text1 value="foobar" required>
<input type=text id=text2 required>
<input type=text id=text3>
<select id=select1 required>
 <optgroup label="options" id=optgroup1>
   <option value="option1" id=option1>option1
</select>
<select id=select2>
 <optgroup label="options" id=optgroup2>
   <option value="option2" id=option2>option2
</select>
<textarea required id=textarea1>textarea1</textarea>
<textarea id=textarea2>textarea2</textarea>

<script>
  testSelectorIdsMatch(":required", ["text1", "text2", "select1", "textarea1"], "':required' matches required <input>s, <select>s and <textarea>s");
  testSelectorIdsMatch(":optional", ["text3", "select2", "textarea2"], "':optional' matches elements <input>s, <select>s and <textarea>s that are not required");

  document.getElementById("text1").removeAttribute("required");
  testSelectorIdsMatch(":required", ["text2", "select1", "textarea1"], "':required' doesn't match elements whose required attribute has been removed");
  testSelectorIdsMatch(":optional", ["text1", "text3", "select2", "textarea2"], "':optional' matches elements whose required attribute has been removed");

  document.getElementById("select2").setAttribute("required", "required");
  testSelectorIdsMatch(":required", ["text2", "select1", "select2", "textarea1"], "':required' matches elements whose required attribute has been added");
  testSelectorIdsMatch(":optional", ["text1", "text3", "textarea2"], "':optional' doesn't match elements whose required attribute has been added");
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.select.required.first_option.placeholder",
      "message": "The first child “option” element of a “select” element with a “required” attribute, and without a “multiple” attribute, and without a “size” attribute whose value is greater than “1”, must have either an empty “value” attribute, or must have no text content. Consider either adding a placeholder option label, or adding a “size” attribute with a value equal to the number of “option” elements.",
      "severity": "Warning",
      "span": {
        "byte_end": 664,
        "byte_start": 655,
        "col": 1,
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
  "source_name": "html/semantics/selectors/pseudo-classes/required-optional.html"
}
```
