# html/semantics/selectors/pseudo-classes/indeterminate.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/selectors/pseudo-classes/indeterminate.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset=utf-8>
<title>Selector: pseudo-classes (:indeterminate)</title>
<link rel="author" title="Denis Ah-Kang" href="mailto:denis@w3.org" id=link1>
<link rel=help href="https://html.spec.whatwg.org/multipage/#pseudo-classes" id=link2>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="utils.js"></script>
<div id="log"></div>
<input type=checkbox id=checkbox1>
<input type=checkbox id=checkbox2>
<input type=radio id=radio1 checked>
<input type=radio name=radiogroup id=radio2>
<input type=radio name=radiogroup id=radio3>
<input type=radio name=group2 id=radio4>
<input type=radio name=group2 id=radio5>
<progress id="progress1"></progress>
<progress id="progress2" value=10></progress>

<script>
  testSelectorIdsMatch(":indeterminate", ["radio2", "radio3", "radio4", "radio5", "progress1"], "':progress' matches <input>s radio buttons whose radio button group contains no checked input and <progress> elements without value attribute");

  document.getElementById("radio2").setAttribute("checked", "checked");
  testSelectorIdsMatch(":indeterminate", ["radio4", "radio5", "progress1"], "dynamically check a radio input in a radio button group");

  document.getElementById("radio4").click();
  testSelectorIdsMatch(":indeterminate", ["progress1"], "click on radio4 which is in the indeterminate state");

  document.getElementById("progress1").setAttribute("value", "20");
  testSelectorIdsMatch(":indeterminate", [], "adding a value to progress1 should put it in a determinate state");

  document.getElementById("progress2").removeAttribute("value");
  testSelectorIdsMatch(":indeterminate", ["progress2"], "removing progress2's value should put it in an indeterminate state");

  document.getElementById("checkbox1").indeterminate = true; // set checkbox1 in the indeterminate state
  testSelectorIdsMatch(":indeterminate", ["checkbox1", "progress2"], "':progress' also matches <input> checkbox whose indeterminate IDL is set to true");
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.progress.value.exceeds_one",
      "message": "The value of the  “value” attribute must be less than or equal to one when the “max” attribute is absent.",
      "severity": "Warning",
      "span": {
        "byte_end": 769,
        "byte_start": 735,
        "col": 1,
        "line": 18
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
  "source_name": "html/semantics/selectors/pseudo-classes/indeterminate.html"
}
```
