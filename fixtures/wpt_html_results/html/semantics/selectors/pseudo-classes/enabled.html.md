# html/semantics/selectors/pseudo-classes/enabled.html

Counts:
- errors: 6
- warnings: 11
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/selectors/pseudo-classes/enabled.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset=utf-8>
<title>Selector: pseudo-classes (:enabled)</title>
<link rel="author" title="Denis Ah-Kang" href="mailto:denis@w3.org" id=link1>
<link rel=help href="https://html.spec.whatwg.org/multipage/#pseudo-classes" id=link2>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="utils.js"></script>
<div id="log"></div>
<a id=link3></a>
<area id=link4></area>
<link id=link5></link>
<a href="http://www.w3.org" id=link6></a>
<area href="http://www.w3.org" id=link7></area>
<link href="http://www.w3.org" id=link8></link>
<button id=button1>button1</button>
<button id=button2 disabled>button2</button>
<input id=input1>
<input id=input2 disabled>
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
<form>
 <p><input type=submit contextmenu=formmenu id=submitbutton></p>
 <menu type=context id=formmenu>
  <!-- historical; these should *not* match -->
  <menuitem command="submitbutton" default id=menuitem1>
  <menuitem command="resetbutton" disabled id=menuitem2>
 </menu>
</form>
<fieldset id=fieldset1></fieldset>
<fieldset disabled id=fieldset2></fieldset>

<script>
  testSelectorIdsMatch(":enabled", ["button1", "input1", "select1", "optgroup1", "option1", "textarea1", "submitbutton", "fieldset1"], "':enabled' elements that are not disabled");
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.area.map_ancestor.missing",
      "message": "The “area” element must have a “map” ancestor.",
      "severity": "Error",
      "span": {
        "byte_end": 445,
        "byte_start": 430,
        "col": 1,
        "line": 11
      }
    },
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “area”.",
      "severity": "Error",
      "span": {
        "byte_end": 452,
        "byte_start": 445,
        "col": 16,
        "line": 11
      }
    },
    {
      "category": "Html",
      "code": "html.link.href.required",
      "message": "A “link” element must have an “href” or “imagesrcset” attribute, or both.",
      "severity": "Warning",
      "span": {
        "byte_end": 468,
        "byte_start": 453,
        "col": 1,
        "line": 12
      }
    },
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “link”.",
      "severity": "Error",
      "span": {
        "byte_end": 475,
        "byte_start": 468,
        "col": 16,
        "line": 12
      }
    },
    {
      "category": "Html",
      "code": "html.area.map_ancestor.missing",
      "message": "The “area” element must have a “map” ancestor.",
      "severity": "Error",
      "span": {
        "byte_end": 558,
        "byte_start": 518,
        "col": 1,
        "line": 14
      }
    },
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “area”.",
      "severity": "Error",
      "span": {
        "byte_end": 565,
        "byte_start": 558,
        "col": 41,
        "line": 14
      }
    },
    {
      "category": "Html",
      "code": "html.link.missing_rel_or_itemprop_or_property",
      "message": "Element “link” is missing one or more of the following attributes: “itemprop”, “property”, “rel”.",
      "severity": "Warning",
      "span": {
        "byte_end": 606,
        "byte_start": 566,
        "col": 1,
        "line": 15
      }
    },
    {
      "category": "Html",
      "code": "html.link.in_body.disallowed_rel",
      "message": "A “link” element must not appear as a descendant of a “body” element unless the “link” element has an “itemprop” attribute or has a “rel” attribute whose value contains “dns-prefetch”, “modulepreload”, “pingback”, “preconnect”, “prefetch”, “preload”, “prerender”, or “stylesheet”.",
      "severity": "Warning",
      "span": {
        "byte_end": 606,
        "byte_start": 566,
        "col": 1,
        "line": 15
      }
    },
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “link”.",
      "severity": "Error",
      "span": {
        "byte_end": 613,
        "byte_start": 606,
        "col": 41,
        "line": 15
      }
    },
    {
      "category": "Html",
      "code": "html.attr.contextmenu.obsolete",
      "message": "The “contextmenu” attribute is obsolete. Use script to handle “contextmenu” event instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 1172,
        "byte_start": 1116,
        "col": 5,
        "line": 31
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “menuitem” not allowed as child of “menu” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 1314,
        "byte_start": 1260,
        "col": 3,
        "line": 34
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “menuitem” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 1314,
        "byte_start": 1260,
        "col": 3,
        "line": 34
      }
    },
    {
      "category": "Html",
      "code": "html.menu.child.not_allowed",
      "message": "Element “menuitem” not allowed as child of “menu” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 1314,
        "byte_start": 1260,
        "col": 3,
        "line": 34
      }
    },
    {
      "category": "Html",
      "code": "html.element.menuitem.obsolete",
      "message": "The “menuitem” element is obsolete. Use script to handle “contextmenu” event instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 1314,
        "byte_start": 1260,
        "col": 3,
        "line": 34
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “menuitem” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 1371,
        "byte_start": 1317,
        "col": 3,
        "line": 35
      }
    },
    {
      "category": "Html",
      "code": "html.element.menuitem.obsolete",
      "message": "The “menuitem” element is obsolete. Use script to handle “contextmenu” event instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 1371,
        "byte_start": 1317,
        "col": 3,
        "line": 35
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
  "source_name": "html/semantics/selectors/pseudo-classes/enabled.html"
}
```
