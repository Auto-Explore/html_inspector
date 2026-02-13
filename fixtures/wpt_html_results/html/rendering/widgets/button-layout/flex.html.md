# html/rendering/widgets/button-layout/flex.html

Counts:
- errors: 0
- warnings: 10
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/widgets/button-layout/flex.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>button with flex/inline-flex</title>
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<style>
#inline-flex { display: inline-flex }
#flex { display: flex }
#ref > div { display: flex }

#flexstart {
  border: none;
  padding: 0;
  display: flex;
  align-items: flex-start;
  height: 3em;
}

#stretch {
  border: none;
  padding: 0;
  display: flex;
  align-items: stretch;
  height: 3em;
}

#no-align-items {
  border: none;
  padding: 0;
  display: flex;
  height: 3em;
}
</style>
<button id=inline-flex><div>1</div><div>2</div><div>3</div><div>4</div></button>
<button id=flex><div>1</div><div>2</div><div>3</div><div>4</div></button>
<button id=ref><div><div>1</div><div>2</div><div>3</div><div>4</div></div></button>

<div><button id="flexstart"><span id="flexstart-item">abc</span></button></div>

<div><button id="stretch"><span id="stretch-item">abc</span></button></div>
<div><button id="no-align-items"><span id="no-align-items-item">abc</span></button></div>
<script>
const ref = document.getElementById('ref');
const expectedWidth = ref.clientWidth;
const expectedHeight = ref.clientHeight;
for (const elm of [document.getElementById('inline-flex'),
                   document.getElementById('flex')]) {
  test(() => {
    // check that flex is supported
    const flexValue = elm.id;
    assert_equals(getComputedStyle(elm).display, flexValue, `${flexValue} is not supported`);
    const width = elm.clientWidth;
    const height = elm.clientHeight;
    assert_equals(width, expectedWidth, 'clientWidth');
    assert_equals(height, expectedHeight, 'clientHeight');
  }, elm.id);
}

// crbug.com/700029
test(() => {
  assert_equals(document.getElementById('flexstart').offsetTop,
    document.getElementById('flexstart-item').offsetTop);
}, 'align-items:flex-start should work');

// crbug.com/1004163
test(() => {
  assert_equals(document.getElementById('stretch').offsetHeight,
    document.getElementById('stretch-item').offsetHeight);
}, 'align-items:stretch should work');

// crbug.com/40681980
test(() => {
  assert_equals(document.getElementById('no-align-items').offsetHeight,
    document.getElementById('no-align-items-item').offsetHeight);
}, 'No align-items should work as stretch');
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
        "byte_end": 586,
        "byte_start": 581,
        "col": 24,
        "line": 33
      }
    },
    {
      "category": "Html",
      "code": "html.child.disallowed_in_phrasing_parent",
      "message": "Element “div” not allowed as child of “button” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 598,
        "byte_start": 593,
        "col": 36,
        "line": 33
      }
    },
    {
      "category": "Html",
      "code": "html.child.disallowed_in_phrasing_parent",
      "message": "Element “div” not allowed as child of “button” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 610,
        "byte_start": 605,
        "col": 48,
        "line": 33
      }
    },
    {
      "category": "Html",
      "code": "html.child.disallowed_in_phrasing_parent",
      "message": "Element “div” not allowed as child of “button” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 622,
        "byte_start": 617,
        "col": 60,
        "line": 33
      }
    },
    {
      "category": "Html",
      "code": "html.child.disallowed_in_phrasing_parent",
      "message": "Element “div” not allowed as child of “button” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 660,
        "byte_start": 655,
        "col": 17,
        "line": 34
      }
    },
    {
      "category": "Html",
      "code": "html.child.disallowed_in_phrasing_parent",
      "message": "Element “div” not allowed as child of “button” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 672,
        "byte_start": 667,
        "col": 29,
        "line": 34
      }
    },
    {
      "category": "Html",
      "code": "html.child.disallowed_in_phrasing_parent",
      "message": "Element “div” not allowed as child of “button” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 684,
        "byte_start": 679,
        "col": 41,
        "line": 34
      }
    },
    {
      "category": "Html",
      "code": "html.child.disallowed_in_phrasing_parent",
      "message": "Element “div” not allowed as child of “button” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 696,
        "byte_start": 691,
        "col": 53,
        "line": 34
      }
    },
    {
      "category": "Html",
      "code": "html.child.disallowed_in_phrasing_parent",
      "message": "Element “div” not allowed as child of “button” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 733,
        "byte_start": 728,
        "col": 16,
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
  "source_name": "html/rendering/widgets/button-layout/flex.html"
}
```
