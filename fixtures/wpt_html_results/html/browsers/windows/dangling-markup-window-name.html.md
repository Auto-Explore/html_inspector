# html/browsers/windows/dangling-markup-window-name.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/windows/dangling-markup-window-name.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<html>
<head>
  <title>Dangling Markup in target</title>
  <meta name="timeout" content="long">
  <script src="/resources/testharness.js"></script>
  <script src="/resources/testharnessreport.js"></script>
  <script src="/common/utils.js"></script>
</head>
<body>
  <script>
    function anchorClick(target, id) {
      const hyperlink = document.body.appendChild(document.createElement('a'));
      if (target) {
        hyperlink.target = target;
      }
      hyperlink.href = `resources/window-name.sub.html?report=${id}|close`;
      hyperlink.click();
    }

    async function pollResultAndCheck(t, id, expected) {
      const stashURL = new URL('resources/window-name-stash.py', location);
      stashURL.searchParams.set('id', id);

      let res = 'NONE';
      while (res == 'NONE') {
        await new Promise(resolve => { t.step_timeout(resolve, 100); });

        const response = await fetch(stashURL);
        res = await response.text();
      }
      if (res !== expected) {
        assert_unreached('Stash result does not equal expected result.')
      }
    }

    promise_test(async t => {
      const id = token();
      const value = '\n<' + id;

      window.open(`resources/window-name.sub.html?report=${id}|close`, value);
      await pollResultAndCheck(t, id, value);
    }, 'Dangling Markup in target is not reset when set by window.open');

    promise_test(async t => {
      const id = token();
      const value = '\n<' + id;

      anchorClick(value, id)
      await pollResultAndCheck(t, id, '');
    }, 'Dangling Markup with "\\n" in target is reset when set by <a> tag');

    promise_test(async t => {
      const id = token();
      const value = '\r<' + id;

      anchorClick(value, id)
      await pollResultAndCheck(t, id, '');
    }, 'Dangling Markup with "\\r" in target is reset when set by <a> tag');

    promise_test(async t => {
      const id = token();
      const value = '\t<' + id;

      anchorClick(value, id)
      await pollResultAndCheck(t, id, '');
    }, 'Dangling Markup with "\\t" in target is reset when set by <a> tag');

    promise_test(async t => {
      const id = token();
      const value = '\n<' + id;

      const form = document.body.appendChild(document.createElement('form'));
      form.target = value;
      form.method = 'GET';
      form.action = 'resources/window-name.sub.html';
      const input = form.appendChild(document.createElement('input'));
      input.type = 'hidden';
      input.name = 'report';
      input.value = `${id}|close`;
      form.submit();

      await pollResultAndCheck(t, id, '');
    }, 'Dangling Markup in target is reset when set by <form> tag');

    promise_test(async t => {
      const id = token();
      const value = '\n<' + id;
      const base = document.head.appendChild(document.createElement('base'));
      base.target = value;

      anchorClick(null, id)
      await pollResultAndCheck(t, id, '');
    }, 'Dangling Markup in target is reset when set by <base> tag');
  </script>
</body>
</html>
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
  "source_name": "html/browsers/windows/dangling-markup-window-name.html"
}
```
