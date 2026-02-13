# html/semantics/selectors/pseudo-classes/checked.html

Counts:
- errors: 0
- warnings: 14
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/selectors/pseudo-classes/checked.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset=utf-8>
<title>Selector: pseudo-classes (:checked)</title>
<link rel="author" title="Denis Ah-Kang" href="mailto:denis@w3.org" id=link1>
<link rel=help href="https://html.spec.whatwg.org/multipage/#pseudo-classes" id=link2>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="utils.js"></script>
<div id="log"></div>
<select id=select1>
 <optgroup label="options" id=optgroup1>
  <option value="option1" id=option1 selected>option1
  <option value="option2" id=option2>option2
  <option value="option2" id=option3 checked>option3
</select>
<input type=checkbox id=checkbox1 checked>
<input type=checkbox id=checkbox2>
<input type=checkbox id=checkbox3 selected>
<input type=radio id=radio1 checked>
<input type=radio id=radio2>
<form>
  <p><input type=submit contextmenu=formmenu id="submitbutton"></p>
  <menu type=context id=formmenu>
    <!-- historical; these should *not* match -->
    <menuitem type=checkbox checked default id=menuitem1>
    <menuitem type=checkbox default id=menuitem2>
    <menuitem type=checkbox id=menuitem3>
    <menuitem type=radio checked id=menuitem4>
    <menuitem type=radio id=menuitem5>
  </menu>
</form>

<script>
  testSelectorIdsMatch(":checked", ["option1", "checkbox1", "radio1"], "':checked' matches checked <input>s in checkbox and radio button states, selected <option>s");

  document.getElementById("checkbox1").removeAttribute("type");  // change type of input
  document.getElementById("radio1").removeAttribute("type");  // change type of input
  testSelectorIdsMatch(":checked", ["option1"], "':checked' should no longer match <input>s whose type checkbox/radio has been removed");

  document.getElementById("option2").selected = "selected";  // select option2
  document.getElementById("checkbox2").click();  // check chekbox2
  document.getElementById("radio2").click();  // check radio2
  testSelectorIdsMatch(":checked", ["option2", "checkbox2", "radio2"], "':checked' matches clicked checkbox and radio buttons");
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.attr.contextmenu.obsolete",
      "message": "The “contextmenu” attribute is obsolete. Use script to handle “contextmenu” event instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 894,
        "byte_start": 836,
        "col": 6,
        "line": 22
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “menuitem” not allowed as child of “menu” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 1040,
        "byte_start": 987,
        "col": 5,
        "line": 25
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “menuitem” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 1040,
        "byte_start": 987,
        "col": 5,
        "line": 25
      }
    },
    {
      "category": "Html",
      "code": "html.menu.child.not_allowed",
      "message": "Element “menuitem” not allowed as child of “menu” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 1040,
        "byte_start": 987,
        "col": 5,
        "line": 25
      }
    },
    {
      "category": "Html",
      "code": "html.element.menuitem.obsolete",
      "message": "The “menuitem” element is obsolete. Use script to handle “contextmenu” event instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 1040,
        "byte_start": 987,
        "col": 5,
        "line": 25
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “menuitem” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 1090,
        "byte_start": 1045,
        "col": 5,
        "line": 26
      }
    },
    {
      "category": "Html",
      "code": "html.element.menuitem.obsolete",
      "message": "The “menuitem” element is obsolete. Use script to handle “contextmenu” event instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 1090,
        "byte_start": 1045,
        "col": 5,
        "line": 26
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “menuitem” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 1132,
        "byte_start": 1095,
        "col": 5,
        "line": 27
      }
    },
    {
      "category": "Html",
      "code": "html.element.menuitem.obsolete",
      "message": "The “menuitem” element is obsolete. Use script to handle “contextmenu” event instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 1132,
        "byte_start": 1095,
        "col": 5,
        "line": 27
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “menuitem” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 1179,
        "byte_start": 1137,
        "col": 5,
        "line": 28
      }
    },
    {
      "category": "Html",
      "code": "html.element.menuitem.obsolete",
      "message": "The “menuitem” element is obsolete. Use script to handle “contextmenu” event instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 1179,
        "byte_start": 1137,
        "col": 5,
        "line": 28
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “menuitem” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 1218,
        "byte_start": 1184,
        "col": 5,
        "line": 29
      }
    },
    {
      "category": "Html",
      "code": "html.element.menuitem.obsolete",
      "message": "The “menuitem” element is obsolete. Use script to handle “contextmenu” event instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 1218,
        "byte_start": 1184,
        "col": 5,
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
  "source_name": "html/semantics/selectors/pseudo-classes/checked.html"
}
```
