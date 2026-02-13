# html/editing/the-hidden-attribute/hidden-ua-stylesheet.html

Counts:
- errors: 1
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/the-hidden-attribute/hidden-ua-stylesheet.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel=author href="mailto:jarhar@chromium.org">
<link rel=help href="https://html.spec.whatwg.org/multipage/rendering.html#hiddenCSS">
<link rel=help href="https://github.com/whatwg/html/pull/7475#issuecomment-1069313217">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<div id=div>hello world</div>
<table id=table>
  <colgroup id=colgroup>
    <col id=col></col>
  </colgroup>
</table>

<script>
function testDisplayNone(description) {
  test(() => {
    assert_equals(getComputedStyle(div).display, 'none',
        `${description} should make the div display:none.`);
    assert_equals(getComputedStyle(div).contentVisibility, 'visible',
        `${description} should not affect the div's content-visibility property.`);
  }, description);
}

function testCVHidden(description) {
  test(() => {
    assert_equals(getComputedStyle(div).display, 'block',
        `${description} should not affect the div's display property.`);
    assert_equals(getComputedStyle(div).contentVisibility, 'hidden',
        `${description} should make the div content-visibility:hidden.`);
    assert_equals(div.hidden, "until-found",
      `${description} should make the div hidden=until-found.`);
  }, description);

}

function testNormal(description) {
  test(() => {
    assert_equals(getComputedStyle(div).display, 'block',
        `${description} should not affect the div's display property.`);
    assert_equals(getComputedStyle(div).contentVisibility, 'visible',
        `${description} should not affect the div's content-visibility property.`);
  }, description);
}

test(() => {
  div.removeAttribute('hidden');
  testNormal(`div.removeAttribute('hidden')`);

  div.setAttribute('hidden', '');
  testDisplayNone(`div.setAttribute('hidden', '')`);

  div.setAttribute('hidden', 'asdf');
  testDisplayNone(`div.setAttribute('hidden', 'asdf')`);

  div.setAttribute('hidden', 'until-found');
  testCVHidden(`div.setAttribute('hidden', 'until-found')`);

  div.setAttribute('hidden', 'UNTIL-FOUND');
  testCVHidden(`div.setAttribute('hidden', 'UNTIL-FOUND')`);

  div.setAttribute('hidden', 'UnTiL-FoUnD');
  testCVHidden(`div.setAttribute('hidden', 'UnTiL-FoUnD')`);

  div.setAttribute('hidden', '0');
  testDisplayNone(`div.setAttribute('hidden', '0')`);
});
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “col”.",
      "severity": "Error",
      "span": {
        "byte_end": 444,
        "byte_start": 438,
        "col": 17,
        "line": 11
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
  "source_name": "html/editing/the-hidden-attribute/hidden-ua-stylesheet.html"
}
```
