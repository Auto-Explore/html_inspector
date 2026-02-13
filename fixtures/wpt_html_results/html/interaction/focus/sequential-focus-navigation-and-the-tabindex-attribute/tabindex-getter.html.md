# html/interaction/focus/sequential-focus-navigation-and-the-tabindex-attribute/tabindex-getter.html

Counts:
- errors: 1
- warnings: 4
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/interaction/focus/sequential-focus-navigation-and-the-tabindex-attribute/tabindex-getter.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>HTML Test: tabIndex getter return value</title>
<link rel="help" href="https://html.spec.whatwg.org/multipage/interaction.html#dom-tabindex">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<style>
#scrollable {
  width: 100px;
  height: 100px;
  overflow: scroll;
}
#scrollable-inner {
  width: 1024px;
  height: 2048px;
}
</style>
<body>
<input>
<input type="hidden">
<button>button</button>
<button disabled>button</button>
<button hidden>button</button>
<a>a</a>
<a href="#">a</a>
<svg><a>svg a</a></svg>
<svg><a>svg a</a></svg>
<link id="nohref">
<textarea></textarea>
<select><optgroup><option>option</option></optgroup></select>
<select multiple></select>
<iframe width="10" height="10"></iframe>
<embed width="10" height="10"></embed>
<object width="10" height="10"></object>
<span></span>
<div></div>
<details><summary>summary</summary><summary id="secondsummary">second summary</summary>details</details>
<div id="hostDelegatesFocus"></div>
<div id="hostNonDelegatesFocus"></div>
<div contenteditable="true"></div>
<div id="scrollable"><div id="scrollable-inner"></div></div>
<fieldset></fieldset>
<output></output>
<slot></slot>
<script>
document.getElementById("hostDelegatesFocus").attachShadow({ mode: "open", delegatesFocus: true });
document.getElementById("hostNonDelegatesFocus").attachShadow({ mode: "open", delegatesFocus: false });
const defaultList = [
  ["input", 0],
  ["input[type='hidden']", 0],
  ["button", 0],
  ["button[disabled]", 0],
  ["button[hidden]", 0],
  ["a", 0],
  ["a[href]", 0],
  ["svg a", 0],
  ["textarea", 0],
  ["select", 0],
  ["select[multiple]", 0],
  ["option", -1],
  ["optgroup", -1],
  ["iframe", 0],
  ["embed", -1],
  ["object", 0],
  ["span", -1],
  ["div", -1],
  ["link#nohref", -1],
  ["link[href]", -1],
  ["details", -1],
  ["summary", 0],
  ["summary#secondsummary", -1],
  ["#hostDelegatesFocus", -1],
  ["#hostNonDelegatesFocus", -1],
  ["div[contenteditable]", -1],
  ["#scrollable", -1],
  ["fieldset", -1],
  ["output", -1],
  ["slot", -1],
];
const tabIndexValue = [-1, 0, 1];
for (const entry of defaultList) {
  const element = document.querySelector(entry[0]);
  test(() => {
    assert_equals(element.tabIndex, entry[1]);
  }, entry[0] + ".tabIndex should return " + entry[1] + " by default");
  for (const setValue of tabIndexValue ) {
    test(() => {
      element.setAttribute("tabindex", setValue);
      assert_equals(element.tabIndex, setValue);
    }, entry[0] + ".tabIndex should return " + setValue + " when set to " + setValue);
  }
}
</script>
</body>

```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.link.href.required",
      "message": "A “link” element must have an “href” or “imagesrcset” attribute, or both.",
      "severity": "Warning",
      "span": {
        "byte_end": 655,
        "byte_start": 637,
        "col": 1,
        "line": 28
      }
    },
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “embed”.",
      "severity": "Error",
      "span": {
        "byte_end": 846,
        "byte_start": 838,
        "col": 31,
        "line": 33
      }
    },
    {
      "category": "Html",
      "code": "html.object.data.missing",
      "message": "Element “object” is missing required attribute “data”.",
      "severity": "Warning",
      "span": {
        "byte_end": 878,
        "byte_start": 847,
        "col": 1,
        "line": 34
      }
    },
    {
      "category": "Html",
      "code": "html.details.multiple_summary",
      "message": "Element “summary” not allowed as child of “details” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 977,
        "byte_start": 949,
        "col": 36,
        "line": 37
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
  "source_name": "html/interaction/focus/sequential-focus-navigation-and-the-tabindex-attribute/tabindex-getter.html"
}
```
