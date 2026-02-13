# html/semantics/document-metadata/the-link-element/link-multiple-load-events.html

Counts:
- errors: 2
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/document-metadata/the-link-element/link-multiple-load-events.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel="author" title="Dom Farolino" href="mailto:dom@chromium.org">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#the-link-element">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/common/utils.js"></script>
<script>
function getTimeoutPromise(t, msg) {
  return new Promise((resolve, reject) => {
    t.step_timeout(() => reject(`Timeout reached: ${msg}`), 2000);
  });
}

// The tests generated from this loop are important to ensure that a link's load
// or error event handler never incurs infinite recursion by virtue of
// re-setting a link element's attribute to the value it already has. This is
// what thwarted the first attempt at fixing https://crbug.com/41436016 in
// Chromium.
const attributes_to_test = ['rel', 'href', 'type', 'media'];
for (const attribute of attributes_to_test) {
  promise_test(async t => {
    const id = token();
    const link = document.createElement('link');
    let count = 0;
    let response = null;

    link.onerror = t.unreached_func('Sheet should load successfully');
    const firstLoadPromise = new Promise(resolve => {
      link.onload = resolve;
    });

    // Set all attributes to a sensible default, so when we re-assign one of
    // them (`attribute`) idempotently later, the value doesn't change.
    link.rel = 'stylesheet';
    link.media = 'all';
    link.type = 'text/css';
    link.href = new URL(`stylesheet.py?id=${id}`, location.href);

    document.head.append(link);
    t.add_cleanup(() => {
      link.remove();
    });

    await Promise.race([firstLoadPromise, getTimeoutPromise(t, 'first resource')]);
    response = await fetch(`stylesheet.py?id=${id}&count=foo`);
    count = await response.text();
    assert_equals(count, '1', "server sees first style sheet request");

    const secondLoadPromise = new Promise((resolve, reject) => {
      link.onload = () => reject('second load event unexpectedly fired');
    });
    const expectedTimeoutPromise = new Promise(resolve => {
      t.step_timeout(resolve, 2000);
    });

    link[attribute] = link[attribute];
    await Promise.race([expectedTimeoutPromise, secondLoadPromise]);
    response = await fetch(`stylesheet.py?id=${id}&count=foo`);
    count = await response.text();
    assert_equals(count, '0',
        "server does not see second request to the same style sheet");
  }, `<link> cannot request the same resource twice by touching the ` +
     `'${attribute}' attribute, if its value never changes`);
}


promise_test(async t => {
  const id = token();
  const link = document.createElement('link');
  link.rel = 'preload';
  link.as = 'style';
  document.head.append(link);
  t.add_cleanup(() => {
    link.remove();
  });

  let count = 0;
  let response = null;

  link.onerror = t.unreached_func('Sheet should load successfully');
  const firstLoadPromise = new Promise(resolve => {
    link.onload = resolve;
  });

  link.href = `stylesheet.py?id=${id}`;
  await Promise.race([firstLoadPromise, getTimeoutPromise(t, 'first resource')]);
  response = await fetch(`stylesheet.py?id=${id}&count=foo`);
  count = await response.text();
  assert_equals(count, '1', "server sees preload request");

  const secondLoadPromise = new Promise(resolve => {
    link.onload = resolve;
  });

  // Switching from `rel=preload` => `rel=stylesheet` triggers the stylesheet
  // processing model. The resource loads from the preload cache and never
  // touches the network, but the load event does fire again.
  link.rel = 'stylesheet';
  await Promise.race([secondLoadPromise,
      getTimeoutPromise(t, 'rel=stylesheet using preload cache')]);
  response = await fetch(`stylesheet.py?id=${id}&count=foo`);
  count = await response.text();
  assert_equals(count, '0', "server does not see second request to the same style sheet");
}, "<link> load event can fire twice for the same href resource, based on " +
   "'rel' attribute mutations");

promise_test(async t => {
  const link = document.createElement('link');
  link.rel = 'stylesheet';
  document.head.append(link);
  t.add_cleanup(() => {
    link.remove();
  });

  link.onerror = t.unreached_func('Sheet should load successfully');
  const firstLoadPromise = new Promise(resolve => {
    link.onload = resolve;
  });

  link.href = 'style.css?first';
  await Promise.race([firstLoadPromise, getTimeoutPromise(t, 'first resource')]);

  const secondLoadPromise = new Promise(resolve => {
    link.onload = resolve;
  });

  link.href = 'style.css?second';
  await Promise.race([secondLoadPromise, getTimeoutPromise(t, 'second resource')]);

  const thirdLoadPromise = new Promise(resolve => {
    link.onload = resolve;
  });

  link.href = 'style.css?third';
  await Promise.race([thirdLoadPromise, getTimeoutPromise(t, 'third resource')]);
}, "<link> load event fires for each DIFFERENT stylesheet it loads");
</script>
</head>
</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “head”.",
      "severity": "Error",
      "span": {
        "byte_end": 4932,
        "byte_start": 4925,
        "col": 1,
        "line": 138
      }
    },
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “html”.",
      "severity": "Error",
      "span": {
        "byte_end": 4940,
        "byte_start": 4933,
        "col": 1,
        "line": 139
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
  "source_name": "html/semantics/document-metadata/the-link-element/link-multiple-load-events.html"
}
```
