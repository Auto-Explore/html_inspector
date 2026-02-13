# html/semantics/selectors/pseudo-classes/disabled.html

Counts:
- errors: 0
- warnings: 5
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/selectors/pseudo-classes/disabled.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset=utf-8>
<title>Selector: pseudo-classes (:disabled)</title>
<link rel="author" title="Denis Ah-Kang" href="mailto:denis@w3.org" id=link1>
<link rel=help href="https://html.spec.whatwg.org/multipage/#pseudo-classes" id=link2>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="utils.js"></script>
<style>
  #input4 {display:none;}
</style>
<div id="log"></div>
<button id=button1 type=submit>button1</button>
<button id=button2 disabled>button2</button>
<input id=input1>
<input id=input2 disabled>
<input id=input3 readonly>
<input id=input4>
<select id=select1>
 <optgroup label="options" id=optgroup1>
  <option value="option1" id=option1 selected>option1
</select>
<select disabled id=select2>
 <optgroup label="options" disabled id=optgroup2>
  <option value="option2" disabled id=option2>option2
</select>
<textarea id=textarea1>textarea1</textarea>
<textarea disabled id=textarea2>textarea2</textarea>
<fieldset id=fieldset1></fieldset>
<fieldset disabled id=fieldset2>
  <legend><input type=checkbox id=club></legend>
  <p><label>Name on card: <input id=clubname required></label></p>
  <p><label>Card number: <input id=clubnum required pattern="[-0-9]+"></label></p>
</fieldset>
<label disabled></label>
<object disabled></object>
<output disabled></output>
<img disabled/>
<meter disabled></meter>
<progress disabled></progress>

<script>
  testSelectorIdsMatch(":disabled", ["button2", "input2", "select2", "optgroup2", "option2", "textarea2", "fieldset2", "clubname", "clubnum"], "':disabled' should match only disabled elements");

  document.getElementById("button2").removeAttribute("disabled");
  testSelectorIdsMatch(":disabled", ["input2", "select2", "optgroup2", "option2", "textarea2", "fieldset2", "clubname", "clubnum"], "':disabled' should not match elements whose disabled attribute has been removed");

  document.getElementById("button1").setAttribute("disabled", "disabled");
  testSelectorIdsMatch(":disabled", ["button1", "input2", "select2", "optgroup2", "option2", "textarea2", "fieldset2", "clubname", "clubnum"], "':disabled' should also match elements whose disabled attribute has been set");

  document.getElementById("button1").setAttribute("disabled", "disabled");
  testSelectorIdsMatch(":disabled", ["button1", "input2", "select2", "optgroup2", "option2", "textarea2", "fieldset2", "clubname", "clubnum"], "':disabled' should also match elements whose disabled attribute has been set twice");

  document.getElementById("input2").setAttribute("type", "submit"); // change input type to submit
  testSelectorIdsMatch(":disabled", ["button1", "input2", "select2", "optgroup2", "option2", "textarea2", "fieldset2", "clubname", "clubnum"], "':disabled' should also match disabled elements whose type has changed");

  var input = document.createElement("input");
  input.setAttribute("disabled", "disabled");
  testSelectorIdsMatch(":disabled", ["button1", "input2", "select2", "optgroup2", "option2", "textarea2", "fieldset2", "clubname", "clubnum"], "':disabled' should not match elements not in the document");

  var fieldset = document.createElement("fieldset");
  fieldset.id = "fieldset_nested";
  fieldset.innerHTML = `
    <input id=input_nested>
    <button id=button_nested>button nested</button>
    <select id=select_nested>
      <optgroup label="options" id=optgroup_nested>
        <option value="options" id=option_nested>option nested</option>
      </optgroup>
    </select>
    <textarea id=textarea_nested>textarea nested</textarea>
    <object id=object_nested></object>
    <output id=output_nested></output>
    <fieldset id=fieldset_nested2>
      <input id=input_nested2>
    </fieldset>
  `;
  document.getElementById("fieldset2").appendChild(fieldset);
  testSelectorIdsMatch("#fieldset2 :disabled", ["clubname", "clubnum", "fieldset_nested", "input_nested", "button_nested", "select_nested", "textarea_nested", "fieldset_nested2", "input_nested2"], "':disabled' should match elements that are appended to a disabled fieldset dynamically");
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.object.data.missing",
      "message": "Element “object” is missing required attribute “data”.",
      "severity": "Warning",
      "span": {
        "byte_end": 1326,
        "byte_start": 1309,
        "col": 1,
        "line": 36
      }
    },
    {
      "category": "Html",
      "code": "html.img.src_or_srcset.missing",
      "message": "Element “img” is missing one or more of the following attributes: “src”, “srcset”.",
      "severity": "Warning",
      "span": {
        "byte_end": 1378,
        "byte_start": 1363,
        "col": 1,
        "line": 38
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 1378,
        "byte_start": 1363,
        "col": 1,
        "line": 38
      }
    },
    {
      "category": "Html",
      "code": "html.meter.missing_value",
      "message": "Element “meter” is missing required attribute “value”.",
      "severity": "Warning",
      "span": {
        "byte_end": 1395,
        "byte_start": 1379,
        "col": 1,
        "line": 39
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
  "source_name": "html/semantics/selectors/pseudo-classes/disabled.html"
}
```
