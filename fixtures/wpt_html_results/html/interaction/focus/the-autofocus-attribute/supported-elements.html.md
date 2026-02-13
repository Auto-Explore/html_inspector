# html/interaction/focus/the-autofocus-attribute/supported-elements.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/interaction/focus/the-autofocus-attribute/supported-elements.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="resources/utils.js"></script>
<script>
"use strict";

promise_test(async t => {
  let w = window.open('/common/blank.html');
  await waitForLoad(w);
  t.add_cleanup(() => { w.close(); });
  w.document.body.innerHTML = '<div contenteditable=true autofocus></div>';
  await waitUntilStableAutofocusState(w);
  assert_equals(w.document.activeElement.tagName, 'DIV');
}, 'Contenteditable element should support autofocus');

promise_test(async t => {
  let w = window.open('/common/blank.html');
  await waitForLoad(w);
  t.add_cleanup(() => { w.close(); });
  w.document.body.innerHTML = '<span tabindex=0 autofocus></span>';
  await waitUntilStableAutofocusState(w);
  assert_equals(w.document.activeElement.tagName, 'SPAN');
}, 'Element with tabindex should support autofocus');

promise_test(async t => {
  let w = window.open('/common/blank.html');
  await waitForLoad(w);
  t.add_cleanup(() => { w.close(); });
  let element = w.document.createElementNS('uri1', 'prefix:local');
  element.setAttribute('autofocus', '');
  w.document.body.appendChild(element);
  await waitUntilStableAutofocusState(w);
  assert_equals(w.document.activeElement.tagName, 'BODY');
}, 'Non-HTMLElement should not support autofocus');

promise_test(async t => {
  let w = window.open('/common/blank.html');
  await waitForLoad(w);
  t.add_cleanup(() => { w.close(); });
  const host = w.document.createElement('div');
  host.autofocus = true;
  const shadow = host.attachShadow({mode:'closed', delegatesFocus:true});
  shadow.appendChild(w.document.createElement('input'));
  w.document.body.appendChild(host);
  await waitUntilStableAutofocusState(w);
  assert_equals(w.document.activeElement, host);
  assert_equals(shadow.activeElement.tagName, 'INPUT');
}, 'Host element with delegatesFocus should support autofocus');

promise_test(async t => {
  let w = window.open('/common/blank.html');
  await waitForLoad(w);
  t.add_cleanup(() => { w.close(); });
  const host = w.document.createElement('div');
  host.autofocus = true;
  host.attachShadow({mode:'closed', delegatesFocus:true});
  w.document.body.appendChild(host);
  const next = w.document.createElement('input');
  next.autofocus = true;
  w.document.body.appendChild(next);
  await waitUntilStableAutofocusState(w);
  assert_equals(w.document.activeElement, next);
}, 'Host element with delegatesFocus including no focusable descendants should be skipped');

promise_test(async t => {
  let w = window.open('./resources/imagemap.html');
  await waitForLoad(w);
  t.add_cleanup(() => { w.close(); });
  const area = w.document.createElement('area');
  area.autofocus = true;
  area.shape = 'rect';
  area.coords = '1,1,99,99';
  area.href = '/common/blank.html';
  w.document.querySelector('map').appendChild(area);
  await waitUntilStableAutofocusState(w);
  // According to the specification, DOM anchor for an AREA shape is an IMG
  // element, but major browsers don't follow it.
  // See https://github.com/whatwg/html/issues/5054
  assert_equals(w.document.activeElement, area);
}, 'Area element should support autofocus');
</script>
```

```json
{
  "messages": [
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
  "source_name": "html/interaction/focus/the-autofocus-attribute/supported-elements.html"
}
```
