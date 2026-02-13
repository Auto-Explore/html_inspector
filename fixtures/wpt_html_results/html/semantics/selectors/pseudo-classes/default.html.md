# html/semantics/selectors/pseudo-classes/default.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/selectors/pseudo-classes/default.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset=utf-8>
<title>Selector: pseudo-classes (:default)</title>
<link rel="author" title="Denis Ah-Kang" href="mailto:denis@w3.org" id=link1>
<link rel=help href="https://html.spec.whatwg.org/multipage/#pseudo-classes" id=link2>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="utils.js"></script>
<div id="log"></div>
<form>
  <button id=button1 type=button>button1</button>
  <button id=button2 type=submit>button2</button>
</form>
<form>
  <button id=button3 type=reset>button3</button>
  <button id=button4>button4</button>
</form>
<button id=button5 type=submit>button5</button>
<form id=form1>
  <input type=text id=input1>
</form>
<input type=text id=input2 form=form1>
<form>
  <input type=submit id=input3>
  <input type=submit id=input4>
</form>
<form>
  <input type=image id=input5>
  <input type=image id=input6>
</form>
<form>
  <input type=submit id=input7>
</form>
<input type=checkbox id=checkbox1 checked>
<input type=checkbox id=checkbox2>
<input type=checkbox id=checkbox3 default>
<input type=radio name=radios id=radio1 checked>
<input type=radio name=radios id=radio2>
<input type=radio name=radios id=radio3 default>
<select id=select1>
 <optgroup label="options" id=optgroup1>
  <option value="option1" id=option1>option1
  <option value="option2" id=option2 selected>option2
</select>
<dialog id="dialog">
  <input type=submit id=input8>
</dialog>
<form>
  <button id=button6 type='invalid'>button6</button>
  <button id=button7>button7</button>
</form>
<form>
  <button id=button8>button8</button>
  <button id=button9>button9</button>
</form>


<script>
  testSelectorIdsMatch(":default", ["button2", "button4", "input3", "input5", "input7", "checkbox1", "radio1", "option2", "button6", "button8"], "':default' matches <button>s that are their form's default button, <input>s of type submit/image that are their form's default button, checked <input>s and selected <option>s");

  document.getElementById("button1").type = "submit"; // change the form's default button
  testSelectorIdsMatch(":default", ["button1", "button4", "input3", "input5", "input7", "checkbox1", "radio1", "option2", "button6", "button8"], "':default' matches dynamically changed form's default buttons");

</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.input.image.alt.missing",
      "message": "Element “input” is missing required attribute “alt”.",
      "severity": "Warning",
      "span": {
        "byte_end": 887,
        "byte_start": 859,
        "col": 3,
        "line": 28
      }
    },
    {
      "category": "Html",
      "code": "html.input.image.alt.missing",
      "message": "Element “input” is missing required attribute “alt”.",
      "severity": "Warning",
      "span": {
        "byte_end": 918,
        "byte_start": 890,
        "col": 3,
        "line": 29
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
  "source_name": "html/semantics/selectors/pseudo-classes/default.html"
}
```
