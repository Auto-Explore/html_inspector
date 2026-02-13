# html/rendering/non-replaced-elements/the-hr-element-0/setting-overflow-visible.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/the-hr-element-0/setting-overflow-visible.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>The hr element: setting 'overflow: visible'</title>
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<style>
/* Use 0 margin for hr instead of default 0.5em to make things simpler */
hr {
  margin-top: 0;
  margin-bottom: 0;
}
.wrapper {
  height: 150px;
  position: relative;
}
#test-visible {
  overflow: visible;
}
.float, hr::before {
  content: "";
  float: left;
  width: 50px;
  height: 50px;
  background-color: orange;
}
.clear {
  clear: left;
}
</style>

<div class=wrapper>
 <div class=float></div>
 <hr id=test-control>
 <div class=float></div>
</div>

<div class=wrapper>
 <div class=float></div>
 <hr id=test-visible>
 <div class=float></div>
</div>

<script>

test(() => {
  const hr = document.getElementById('test-control');
  assert_equals(hr.offsetLeft, 50, 'hr.offsetLeft');
  assert_equals(hr.offsetTop, 0, 'hr.offsetTop');
  assert_equals(hr.clientHeight, 50, 'hr.clientHeight');
  const divAfter = hr.nextElementSibling;
  assert_equals(divAfter.offsetLeft, 0, 'divAfter.offsetLeft');
  assert_equals(divAfter.offsetTop, 50 + 1 + 1 /* hr border */, 'divAfter.offsetTop');
}, 'control');

test(() => {
  const hr = document.getElementById('test-visible');
  assert_equals(hr.offsetLeft, 0, 'hr.offsetLeft');
  assert_equals(hr.offsetTop, 0, 'hr.offsetTop');
  assert_equals(hr.clientHeight, 0, 'hr.clientHeight');
  const divAfter = hr.nextElementSibling;
  assert_equals(divAfter.offsetLeft, 50 + 50, 'divAfter.offsetLeft');
  assert_equals(divAfter.offsetTop, 1 + 1 /* hr border */, 'divAfter.offsetTop');
}, 'overflow: visible');

</script>
```

```json
{
  "messages": [
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/rendering/non-replaced-elements/the-hr-element-0/setting-overflow-visible.html"
}
```
