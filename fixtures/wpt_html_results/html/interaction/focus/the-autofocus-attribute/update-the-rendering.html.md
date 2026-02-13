# html/interaction/focus/the-autofocus-attribute/update-the-rendering.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/interaction/focus/the-autofocus-attribute/update-the-rendering.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<link rel="help" href="https://html.spec.whatwg.org/C/#update-the-rendering">

<body>
<script>
'use strict';

async_test(t => {
  t.events = [];

  let w = window.open('/common/blank.html', 'name',
      'width=100,height=100,menubar=no,toolbar=no,location=no');
  t.add_cleanup(() => { w.close(); });
  w.addEventListener('load', t.step_func(() => {
    w.focus();
    let element = w.document.createElement('input');
    element.autofocus = true;
    element.style.marginTop = '200px'; // Setting focus causes scrolling.
    element.addEventListener('focus', t.step_func(() => {
      t.events.push('autofocus');
    }));

    w.addEventListener('scroll', t.step_func(() => {
      t.events.push('scroll');
    }));

    w.requestAnimationFrame(
        () => w.requestAnimationFrame(t.step_func_done(() => {
      t.events.push('animationFrame');
      assert_array_equals(t.events, ['autofocus', 'scroll', 'animationFrame'], t.events);
    })));

    w.document.body.appendChild(element);
  }));
}, '"Flush autofocus candidates" should be happen before a scroll event and ' +
    'animation frame callbacks');
</script>
</body>
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
  "source_name": "html/interaction/focus/the-autofocus-attribute/update-the-rendering.html"
}
```
