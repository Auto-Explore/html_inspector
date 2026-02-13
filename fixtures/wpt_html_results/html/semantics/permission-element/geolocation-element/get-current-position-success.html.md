# html/semantics/permission-element/geolocation-element/get-current-position-success.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/permission-element/geolocation-element/get-current-position-success.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset="utf-8" />
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js?feature=bidi"></script>
<script src="/resources/testdriver-vendor.js"></script>

<script>
  promise_setup(async () => {
    // Ensure permission is granted before proceeding.
    await test_driver.bidi.permissions.set_permission({
      descriptor: { name: 'geolocation' },
      state: 'granted',
    });
  });

  promise_test(async (t) => {
    t.add_cleanup(async () => {
      await test_driver.bidi.emulation.set_geolocation_override({ coordinates: null });
    });

    const latitude = 50;
    const longitude = 0;
    const accuracy = 100;

    await test_driver.bidi.emulation.set_geolocation_override({
      coordinates: { latitude, longitude, accuracy },
    });
    document.innerHTML +=
      '<geolocation id="geolocation-element" autolocate onlocation="onLocation()"></geolocation>';
  }, 'Tests Geolocation element\'s success callback');

  function onLocation() {
    let el = document.getElementById('geolocation-element');
    assert_equals(el.position.coords.latitude, 50);
    assert_equals(el.position.coords.longitude, 0);
    assert_equals(el.position.coords.accuracy, 100);
    assert_equals(el.error, null);
  }
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
  "source_name": "html/semantics/permission-element/geolocation-element/get-current-position-success.html"
}
```
