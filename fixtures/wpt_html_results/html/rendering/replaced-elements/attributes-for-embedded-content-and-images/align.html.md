# html/rendering/replaced-elements/attributes-for-embedded-content-and-images/align.html

Counts:
- errors: 0
- warnings: 4
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/replaced-elements/attributes-for-embedded-content-and-images/align.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>align attribute mapping on replaced elements</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<img id="replaced" src="/images/green.png">
<something id="non-replaced"></something>
<script>
const kMapping = {
  "left": ["float", "left"],
  "right": ["float", "right"],

  "top": ["vertical-align", "top"],

  // This one requires a magic value (`-moz-middle-with-baseline` on Gecko,
  // `-webkit-baseline-middle` on WebKit-based browsers).
  "middle": ["vertical-align", undefined],
  // These are inconsistent across browsers. WebKit maps it to "middle", Gecko
  // to the aforementioned value.
  "center": ["vertical-align", undefined],

  "baseline": ["vertical-align", "baseline"],
  "bottom": ["vertical-align", "baseline"], // *shrug*

  "texttop": ["vertical-align", "text-top"],
  "absmiddle": ["vertical-align", "middle"],
  "abscenter": ["vertical-align", "middle"],
  "absbottom": ["vertical-align", "bottom"],
};

const kInitialValues = {
  "vertical-align": "baseline",
  "float": "none",
};

let replaced = document.getElementById("replaced");
let nonReplaced = document.getElementById("non-replaced");
let t = async_test("align attribute mapping");
onload = t.step_func_done(function() {
  for (const attributeValue in kMapping) {
    for (const element of [replaced, nonReplaced]) {
      test(function() {
        element.setAttribute("align", attributeValue);
        let [property, expected] = kMapping[attributeValue];
        let actual = getComputedStyle(element).getPropertyValue(property);
        if (element == nonReplaced) {
          assert_equals(actual, kInitialValues[property], "align shouldn't map to non-replaced elements")
        } else {
          if (expected) {
            assert_equals(actual, expected, `align=${attributeValue} should map to ${property}: ${expected}`);
          } else {
            assert_equals(property, "vertical-align");
            assert_not_equals(actual, "baseline", `align=${attributeValue} should map a vertical-align value`);
          }
        }
      }, `align=${attributeValue} on ${element == replaced ? "replaced" : "non-replaced"} elements`);
    }
  }
});
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 225,
        "byte_start": 182,
        "col": 1,
        "line": 5
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “something” not allowed as child of “body” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 255,
        "byte_start": 226,
        "col": 1,
        "line": 6
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “something” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 255,
        "byte_start": 226,
        "col": 1,
        "line": 6
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
  "source_name": "html/rendering/replaced-elements/attributes-for-embedded-content-and-images/align.html"
}
```
