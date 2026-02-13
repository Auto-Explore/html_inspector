# html/rendering/widgets/button-layout/grid.html

Counts:
- errors: 0
- warnings: 10
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/widgets/button-layout/grid.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>button with grid/inline-grid</title>
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<style>
#inline-grid { display: inline-grid }
#grid { display: grid }
#ref > div { display: grid }
#inline-grid, #grid, #ref > div { grid-template: auto auto / auto auto }
</style>
<button id=inline-grid><div>1</div><div>2</div><div>3</div><div>4</div></button>
<button id=grid><div>1</div><div>2</div><div>3</div><div>4</div></button>
<button id=ref><div><div>1</div><div>2</div><div>3</div><div>4</div></div></button>
<script>
const ref = document.getElementById('ref');
const expectedWidth = ref.clientWidth;
const expectedHeight = ref.clientHeight;
for (const elm of [document.getElementById('inline-grid'),
                   document.getElementById('grid')]) {
  test(() => {
    // check that grid is supported
    const gridValue = elm.id;
    assert_equals(getComputedStyle(elm).display, gridValue, `${gridValue} is not supported`);
    const width = elm.clientWidth;
    const height = elm.clientHeight;
    assert_equals(width, expectedWidth, 'clientWidth');
    assert_equals(height, expectedHeight, 'clientHeight');
  }, elm.id);
}
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.child.disallowed_in_phrasing_parent",
      "message": "Element “div” not allowed as child of “button” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 371,
        "byte_start": 366,
        "col": 24,
        "line": 11
      }
    },
    {
      "category": "Html",
      "code": "html.child.disallowed_in_phrasing_parent",
      "message": "Element “div” not allowed as child of “button” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 383,
        "byte_start": 378,
        "col": 36,
        "line": 11
      }
    },
    {
      "category": "Html",
      "code": "html.child.disallowed_in_phrasing_parent",
      "message": "Element “div” not allowed as child of “button” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 395,
        "byte_start": 390,
        "col": 48,
        "line": 11
      }
    },
    {
      "category": "Html",
      "code": "html.child.disallowed_in_phrasing_parent",
      "message": "Element “div” not allowed as child of “button” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 407,
        "byte_start": 402,
        "col": 60,
        "line": 11
      }
    },
    {
      "category": "Html",
      "code": "html.child.disallowed_in_phrasing_parent",
      "message": "Element “div” not allowed as child of “button” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 445,
        "byte_start": 440,
        "col": 17,
        "line": 12
      }
    },
    {
      "category": "Html",
      "code": "html.child.disallowed_in_phrasing_parent",
      "message": "Element “div” not allowed as child of “button” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 457,
        "byte_start": 452,
        "col": 29,
        "line": 12
      }
    },
    {
      "category": "Html",
      "code": "html.child.disallowed_in_phrasing_parent",
      "message": "Element “div” not allowed as child of “button” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 469,
        "byte_start": 464,
        "col": 41,
        "line": 12
      }
    },
    {
      "category": "Html",
      "code": "html.child.disallowed_in_phrasing_parent",
      "message": "Element “div” not allowed as child of “button” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 481,
        "byte_start": 476,
        "col": 53,
        "line": 12
      }
    },
    {
      "category": "Html",
      "code": "html.child.disallowed_in_phrasing_parent",
      "message": "Element “div” not allowed as child of “button” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 518,
        "byte_start": 513,
        "col": 16,
        "line": 13
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
  "source_name": "html/rendering/widgets/button-layout/grid.html"
}
```
