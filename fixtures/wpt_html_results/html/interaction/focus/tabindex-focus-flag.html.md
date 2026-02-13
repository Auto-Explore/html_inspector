# html/interaction/focus/tabindex-focus-flag.html

Counts:
- errors: 3
- warnings: 14
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/interaction/focus/tabindex-focus-flag.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel="help" href="https://html.spec.whatwg.org/multipage/interaction.html#specially-focusable">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<body>
<div id="default-samples">
<a></a>
<a href=""></a>
<button></button>
<input type="hidden">
<input type="button">
<select><option>abc</option></select>
<textarea></textarea>
<summary id="summary-out"></summary>
<details open><summary id="summary-first"></summary><summary id="summary-second"></summary></details>
<div contenteditable="true"></div>
<iframe></iframe>
<svg><a id="svg-a"></a></svg>
<svg><text id="svg-text"></text></svg>
<img>
</div>
<script>
setup({ explicit_done: true });
window.addEventListener("load", runTests);

function runTests() {
  const defaultList = [
    ['a', false],
    ['a[href]', true],
    ['button', true],
    ['input[type="hidden"]', false],
    ['input[type="button"]', true],
    ['select', true],
    ['textarea', true],
    ['#summary-out', false],
    ['#summary-first', true],
    ['#summary-second', false],
    ['[contenteditable]', true],
    ['iframe', true],
    ['#svg-a', false],
    ['#svg-text', false],
    ['img', false],
  ];
  for (entry of defaultList) {
    test(() => {
      var element = document.querySelector('#default-samples ' + entry[0]);
      element.focus();
      if (entry[1])
        assert_equals(document.activeElement, element);
      else
        assert_not_equals(document.activeElement, element);
    }, entry[0] + ' should ' + (entry[1] ? '' : 'not ') + 'be focusable by default.');
  }

  runTests_tabindex0();
}
</script>

<div id="tabindex-0">
<a tabindex="0"></a>
<svg><a tabindex="0"></a></svg>
<svg><text tabindex="0"></text></svg>
<summary tabindex="0" id="summary-out-tabindex0"></summary>
<details open><summary id="summary-first"></summary><summary tabindex="0" id="summary-second-tabindex0"></summary></details>
<img tabindex="0">
</div>
<script>
function runTests_tabindex0() {
  for (element of document.querySelectorAll('#tabindex-0 [tabindex]')) {
    var elementDesc = element.tagName;
    if (element.id)
      elementDesc += '#' + element.id;
    test(() => {
      element.focus();
      assert_equals(document.activeElement, element);
    }, elementDesc + ' with tabindex=0 should be focusable.');
  }

  runTests_tabindex_negative();
}
</script>

<div id="tabindex-negative">
<a tabindex="-1"></a>
<svg><a tabindex="-1"></a></svg>
<svg><text tabindex="-1"></text></svg>
<summary tabindex="-1" id="summary-out-tabindex-negative"></summary>
<details open><summary id="summary-first"></summary><summary tabindex="0" id="summary-second-tabindex-negative"></summary></details>
<img tabindex="-1">
</div>
<script>
function runTests_tabindex_negative() {
  for (element of document.querySelectorAll('#tabindex-negative [tabindex]')) {
    var elementDesc = element.tagName;
    if (element.id)
      elementDesc += '#' + element.id;
    test(() => {
      element.focus();
      assert_equals(document.activeElement, element);
    }, elementDesc + ' with tabindex=-1 should be focusable.');
  }

  runTests_tabindex_invalid();
}
</script>

<div id="tabindex-invalid">
<a tabindex="invalid"></a>
<a href="#" tabindex="invalid" id="with-href" data-focusable=true></a>
<svg><a tabindex="invalid"></a></svg>
<svg><a href="#" tabindex="invalid" id="with-href" data-focusable=true></a></svg>
<svg><text tabindex="invalid"></text></svg>
<div tabindex="invalid"></div>
<summary tabindex="invalid" id="summary-out-tabindex-invalid"></summary>
<img tabindex="invalid">
</div>
<script>
function runTests_tabindex_invalid() {
  for (element of document.querySelectorAll('#tabindex-invalid [tabindex]')) {
    var focusable = element.dataset && element.dataset.focusable;
    var elementDesc = element.tagName;
    if (element.id)
      elementDesc += '#' + element.id;
    test(() => {
      element.focus();
      focusable ? assert_equals(document.activeElement, element)
                : assert_not_equals(document.activeElement, element);
    }, `${elementDesc} with tabindex=invalid should ${focusable ? "be" : "not be"} focusable.`);
  }

  done();
}
</script>
</body>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.input.button.value.nonempty",
      "message": "Element “input” with attribute “type” whose value is “button” must have non-empty attribute “value”.",
      "severity": "Warning",
      "span": {
        "byte_end": 342,
        "byte_start": 321,
        "col": 1,
        "line": 11
      }
    },
    {
      "category": "Html",
      "code": "html.details.multiple_summary",
      "message": "Element “summary” not allowed as child of “details” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 521,
        "byte_start": 492,
        "col": 53,
        "line": 15
      }
    },
    {
      "category": "Html",
      "code": "html.img.src_or_srcset.missing",
      "message": "Element “img” is missing one or more of the following attributes: “src”, “srcset”.",
      "severity": "Warning",
      "span": {
        "byte_end": 669,
        "byte_start": 664,
        "col": 1,
        "line": 20
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 669,
        "byte_start": 664,
        "col": 1,
        "line": 20
      }
    },
    {
      "category": "Html",
      "code": "html.id.duplicate",
      "message": "Duplicate ID “summary-first”.",
      "severity": "Error",
      "span": {
        "byte_end": 1848,
        "byte_start": 1820,
        "col": 15,
        "line": 64
      }
    },
    {
      "category": "Html",
      "code": "html.details.multiple_summary",
      "message": "Element “summary” not allowed as child of “details” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 1910,
        "byte_start": 1858,
        "col": 53,
        "line": 64
      }
    },
    {
      "category": "Html",
      "code": "html.img.src_or_srcset.missing",
      "message": "Element “img” is missing one or more of the following attributes: “src”, “srcset”.",
      "severity": "Warning",
      "span": {
        "byte_end": 1949,
        "byte_start": 1931,
        "col": 1,
        "line": 65
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 1949,
        "byte_start": 1931,
        "col": 1,
        "line": 65
      }
    },
    {
      "category": "Html",
      "code": "html.id.duplicate",
      "message": "Duplicate ID “summary-first”.",
      "severity": "Error",
      "span": {
        "byte_end": 2610,
        "byte_start": 2582,
        "col": 15,
        "line": 88
      }
    },
    {
      "category": "Html",
      "code": "html.details.multiple_summary",
      "message": "Element “summary” not allowed as child of “details” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 2680,
        "byte_start": 2620,
        "col": 53,
        "line": 88
      }
    },
    {
      "category": "Html",
      "code": "html.img.src_or_srcset.missing",
      "message": "Element “img” is missing one or more of the following attributes: “src”, “srcset”.",
      "severity": "Warning",
      "span": {
        "byte_end": 2720,
        "byte_start": 2701,
        "col": 1,
        "line": 89
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 2720,
        "byte_start": 2701,
        "col": 1,
        "line": 89
      }
    },
    {
      "category": "Html",
      "code": "html.id.duplicate",
      "message": "Duplicate ID “with-href”.",
      "severity": "Error",
      "span": {
        "byte_end": 3397,
        "byte_start": 3331,
        "col": 6,
        "line": 111
      }
    },
    {
      "category": "Html",
      "code": "html.img.src_or_srcset.missing",
      "message": "Element “img” is missing one or more of the following attributes: “src”, “srcset”.",
      "severity": "Warning",
      "span": {
        "byte_end": 3580,
        "byte_start": 3556,
        "col": 1,
        "line": 115
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 3580,
        "byte_start": 3556,
        "col": 1,
        "line": 115
      }
    },
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
  "source_name": "html/interaction/focus/tabindex-focus-flag.html"
}
```
