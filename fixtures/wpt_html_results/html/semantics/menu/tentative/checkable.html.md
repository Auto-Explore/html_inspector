# html/semantics/menu/tentative/checkable.html

Counts:
- errors: 0
- warnings: 26
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/menu/tentative/checkable.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<link rel=author href=mailto:dom@chromium.org>
<link rel=help href=https://open-ui.org/components/menu.explainer>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<menubar>
  <fieldset checkable=single>
    <menuitem id=menubar-menuitem></menuitem>
  </fieldset>
</menubar>

<menulist>
  <menuitem id=menulist-menuitem-not-checkable></menuitem>
  <fieldset checkable=single>
    <menuitem id=menulist-menuitem-checkable></menuitem>
  </fieldset>
  <fieldset checkable=single>
    <menuitem id=single1></menuitem>
    <menuitem id=single2></menuitem>
    <menuitem id=single3></menuitem>
  </fieldset>
  <fieldset checkable=multiple>
    <menuitem id=multiple1></menuitem>
    <menuitem id=multiple2></menuitem>
    <menuitem id=multiple3></menuitem>
  </fieldset>
  <fieldset checkable=multiple>
    <menuitem defaultchecked id=defaultCheckedItem>Default checked</menuitem>
  </fieldset>
</menulist>

<script>
test(() => {
  const mi = document.createElement('menuitem');
  assert_false(mi.checked, "defaults to unchecked when detached");
  assert_false(mi.matches(":checked"));
  mi.checked = true;
  assert_equals(mi.checked, false, "cannot be checked when detached");
  assert_false(mi.matches(":checked"));

  document.body.append(mi);

  assert_equals(mi.checked, false, "defaults to unchecked when connected");
  assert_false(mi.matches(":checked"));
  mi.checked = true;
  assert_equals(mi.checked, false,
      "cannot be checked when not attached in a menulist");
  assert_false(mi.matches(":checked"));

  mi.remove();
}, "menuitem is not checkable on its own");

test(() => {
  const mi = document.querySelector("#menubar-menuitem");
  assert_equals(mi.checked, false, "menuitem in menubar is not checked");
  assert_false(mi.matches(":checked"));
  mi.checked = true;
  assert_equals(mi.checked, false, "menuitem in menubar cannot be checked");
  assert_false(mi.matches(":checked"));
}, "menuitem in menubar is not checkable");

test(() => {
  const nonCheckable =
      document.querySelector("#menulist-menuitem-not-checkable");
  assert_equals(nonCheckable.checked, false,
      "menuitem direct child of menulist is not checked");
  assert_false(nonCheckable.matches(":checked"));
  nonCheckable.checked = true;
  assert_equals(nonCheckable.checked, false,
      "menuitem direct child of menulist cannot be checked");
  assert_false(nonCheckable.matches(":checked"));

  const checkable = document.querySelector("#menulist-menuitem-checkable");
  assert_equals(nonCheckable.checked, false,
      "checkable menuitem is not checked");
  assert_false(nonCheckable.matches(":checked"));
  checkable.checked = true;
  assert_equals(checkable.checked, true, "checkable menuitem is checked");
  assert_true(checkable.matches(":checked"));
}, "menuitem in menulist");

test(() => {
  assert_false(single1.checked, "single1 not checked");
  assert_false(single1.matches(":checked"), "single1 not :checked");
  assert_false(single2.checked, "single2 not checked");
  assert_false(single2.matches(":checked"), "single2 not :checked");
  assert_false(single3.checked, "single3 not checked");
  assert_false(single3.matches(":checked"), "single3 not :checked");

  single1.checked = true;
  assert_true(single1.checked, "single1 IS checked [after single1 checked]");
  assert_true(single1.matches(":checked"),
      "single1 IS :checked [after single1 checked]");
  assert_false(single2.checked, "single2 not checked [after single1 checked]");
  assert_false(single2.matches(":checked"),
      "single2 not :checked [after single1 checked]");
  assert_false(single3.checked, "single3 not checked [after single1 checked]");
  assert_false(single3.matches(":checked"),
      "single3 not :checked [after single1 checked]");

  single2.checked = true;
  assert_false(single1.checked, "single1 not checked [after single2 checked]");
  assert_false(single1.matches(":checked"),
      "single1 not :checked [after single2 checked]");
  assert_true(single2.checked, "single2 IS checked [after single1 checked]");
  assert_true(single2.matches(":checked"),
      "single2 IS :checked [after single2 checked]");
  assert_false(single3.checked, "single3 not checked [after single2 checked]");
  assert_false(single3.matches(":checked"),
      "single3 not :checked [after single2 checked]");

  single3.checked = true;
  assert_false(single1.checked, "single1 not checked [after single3 checked]");
  assert_false(single1.matches(":checked"),
      "single1 not :checked [after single3 checked]");
  assert_false(single2.checked, "single2 not checked [after single3 checked]");
  assert_false(single2.matches(":checked"),
      "single2 not :checked [after single3 checked]");
  assert_true(single3.checked, "single3 IS checked [after single3 checked]");
  assert_true(single3.matches(":checked"),
      "single3 IS :checked [after single3 checked]");
}, "checkable menuitem exclusivity");

test(() => {
  assert_false(multiple1.checked, "multiple1 not checked");
  assert_false(multiple1.matches(":checked"), "multiple1 not :checked");
  assert_false(multiple2.checked, "multiple2 not checked");
  assert_false(multiple2.matches(":checked"), "multiple2 not :checked");
  assert_false(multiple3.checked, "multiple3 not checked");
  assert_false(multiple3.matches(":checked"), "multiple2 not :checked");

  multiple1.checked = true;
  assert_true(multiple1.checked, "multiple1 checked [after multiple1 checked]");
  assert_true(multiple1.matches(":checked"),
      "multiple1 IS :checked [after multiple1 checked]");
  assert_false(multiple2.checked, "multiple2 not checked [after multiple1 checked]");
  assert_false(multiple2.matches(":checked"),
      "multiple2 not :checked [after multiple1 checked]");
  assert_false(multiple3.checked, "multiple3 not checked [after multiple1 checked]");
  assert_false(multiple3.matches(":checked"),
      "multiple3 not :checked [after multiple1 checked]");

  multiple2.checked = true;
  assert_true(multiple1.checked, "multiple1 checked [after multiple2 checked]");
  assert_true(multiple1.matches(":checked"),
      "multiple1 IS :checked [after multiple2 checked]");
  assert_true(multiple2.checked, "multiple2 checked [after multiple2 checked]");
  assert_true(multiple2.matches(":checked"),
      "multiple2 IS :checked [after multiple2 checked]");
  assert_false(multiple3.checked, "multiple3 not checked [after multiple2 checked]");
  assert_false(multiple3.matches(":checked"),
      "multiple3 not :checked [after multiple2 checked]");

  multiple3.checked = true;
  assert_true(multiple1.checked, "multiple1 checked [after multiple3 checked]");
  assert_true(multiple1.matches(":checked"),
      "multiple1 IS :checked [after multiple3 checked]");
  assert_true(multiple2.checked, "multiple2 checked [after multiple3 checked]");
  assert_true(multiple2.matches(":checked"),
      "multiple2 IS :checked [after multiple3 checked]");
  assert_true(multiple3.checked, "multiple3 checked [after multiple3 checked]");
  assert_true(multiple3.matches(":checked"),
      "multiple3 IS checked [after multiple3 checked]");
}, "checkable multiple");

test(() => {
  const menulist = document.createElement('menulist');
  const fieldset = menulist.appendChild(document.createElement('fieldset'));
  fieldset.setAttribute("checkable", "single");
  const single1 = fieldset.appendChild(document.createElement('menuitem'));
  const single2 = fieldset.appendChild(document.createElement('menuitem'));

  single1.checked = true;
  single2.checked = true;

  assert_false(single1.checked, "single1 is unchecked");
  assert_true(single2.checked, "single2 is checked");
}, "checkable menuitem exclusivity when disconnected");

test(() => {
  const menulist = document.createElement('menulist');
  const fieldset = menulist.appendChild(document.createElement('fieldset'));
  fieldset.setAttribute("checkable", "single");
  const single1 = fieldset.appendChild(document.createElement('menuitem'));

  single1.checked = true;
  assert_true(single1.checked, true);
  fieldset.removeAttribute('checkable');
  assert_false(single1.checked,
      "menuitem gets unchecked after fieldset becomes uncheckable");
  single1.checked = true;
  assert_false(single1.checked,
      "menuitem cannot become checked after fieldset becomes uncheckable");
}, "when fieldset becomes uncheckable, so do its menuitems");

test(() => {
  const menulist = document.createElement('menulist');
  const fieldset = menulist.appendChild(document.createElement('fieldset'));
  fieldset.setAttribute("checkable", "multiple");
  const single1 = fieldset.appendChild(document.createElement('menuitem'));
  const single2 = fieldset.appendChild(document.createElement('menuitem'));

  single1.checked = true;
  single2.checked = true;
  fieldset.setAttribute("checkable", "single");
  assert_true(single1.checked, "first menuitem stays checked");
  assert_false(single2.checked, "second menuitem becomes unchecked");
}, "fieldset multiple => single; all but the first checked menuitem gets " +
   "reset");

// Testing the `defaultchecked` content attribute and `defaultChecked` IDL
// attribute.
test(() => {
  assert_true(defaultCheckedItem.checked,
      "defaultchecked makes the menu item checked by default");
  assert_true(defaultCheckedItem.matches(":default"),
      ":default matches when the defaultchecked attribute is present");

  // Unset the default while checkedness is still clean.
  defaultCheckedItem.removeAttribute('defaultchecked');
  assert_false(defaultCheckedItem.defaultChecked,
      "defaultChecked IDL attribute reflects the content attribute");
  assert_false(defaultCheckedItem.checked,
      "Unsetting defaultchecked when checkedness is clean unchecks the item");
  assert_false(defaultCheckedItem.matches(":default"),
      ":default does not match when defaultchecked attribute is missing or false (1/2)");

  // Toggle it again while checkedness is clean, but this time with the IDL
  // attribute, which should reflect the content attribute.
  defaultCheckedItem.defaultChecked = true;
  assert_true(defaultCheckedItem.checked,
      "Setting defaultChecked to true when checkedness is clean checks the item");
  assert_true(defaultCheckedItem.matches(":default"),
      ":default matches when the defaultchecked attribute is set, even via IDL");

  // Unset checkedness, dirtying the default checkedness.
  defaultCheckedItem.checked = false;
  assert_true(defaultCheckedItem.defaultChecked,
      "Default checkedness is still true");
  assert_false(defaultCheckedItem.checked, "checked becomes false");
  assert_true(defaultCheckedItem.matches(":default"),
      "still matches the default checkedness");

  // Unset default checkedness. This will only have the effect of changing the
  // :default pseudo-class matching.
  defaultCheckedItem.defaultChecked = false;
  assert_false(defaultCheckedItem.matches(":default"),
      ":default no longer matches");

  // Set default checkedness back to true. Since the checkedness is now dirty,
  // this won't change the actual checked state anymore, as the menu item has
  // had its default checkedness overridden.
  defaultCheckedItem.defaultChecked = true;
  assert_false(defaultCheckedItem.checked,
      "checked state is no longer dictated by default checkedness any");
  assert_true(defaultCheckedItem.matches(":default"),
      ":default matches again, following the default checkedness, even " +
      "though the checkedness is dirty");
}, "Default checkedness and checkedness dirtying are wired up correctly");
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “menubar” not allowed as child of “body” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 253,
        "byte_start": 244,
        "col": 1,
        "line": 8
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “menubar” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 253,
        "byte_start": 244,
        "col": 1,
        "line": 8
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “menuitem” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 318,
        "byte_start": 288,
        "col": 5,
        "line": 10
      }
    },
    {
      "category": "Html",
      "code": "html.element.menuitem.obsolete",
      "message": "The “menuitem” element is obsolete. Use script to handle “contextmenu” event instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 318,
        "byte_start": 288,
        "col": 5,
        "line": 10
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “menulist” not allowed as child of “body” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 366,
        "byte_start": 356,
        "col": 1,
        "line": 14
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “menulist” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 366,
        "byte_start": 356,
        "col": 1,
        "line": 14
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “menuitem” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 414,
        "byte_start": 369,
        "col": 3,
        "line": 15
      }
    },
    {
      "category": "Html",
      "code": "html.element.menuitem.obsolete",
      "message": "The “menuitem” element is obsolete. Use script to handle “contextmenu” event instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 414,
        "byte_start": 369,
        "col": 3,
        "line": 15
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “menuitem” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 501,
        "byte_start": 460,
        "col": 5,
        "line": 17
      }
    },
    {
      "category": "Html",
      "code": "html.element.menuitem.obsolete",
      "message": "The “menuitem” element is obsolete. Use script to handle “contextmenu” event instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 501,
        "byte_start": 460,
        "col": 5,
        "line": 17
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “menuitem” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 582,
        "byte_start": 561,
        "col": 5,
        "line": 20
      }
    },
    {
      "category": "Html",
      "code": "html.element.menuitem.obsolete",
      "message": "The “menuitem” element is obsolete. Use script to handle “contextmenu” event instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 582,
        "byte_start": 561,
        "col": 5,
        "line": 20
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “menuitem” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 619,
        "byte_start": 598,
        "col": 5,
        "line": 21
      }
    },
    {
      "category": "Html",
      "code": "html.element.menuitem.obsolete",
      "message": "The “menuitem” element is obsolete. Use script to handle “contextmenu” event instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 619,
        "byte_start": 598,
        "col": 5,
        "line": 21
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “menuitem” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 656,
        "byte_start": 635,
        "col": 5,
        "line": 22
      }
    },
    {
      "category": "Html",
      "code": "html.element.menuitem.obsolete",
      "message": "The “menuitem” element is obsolete. Use script to handle “contextmenu” event instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 656,
        "byte_start": 635,
        "col": 5,
        "line": 22
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “menuitem” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 741,
        "byte_start": 718,
        "col": 5,
        "line": 25
      }
    },
    {
      "category": "Html",
      "code": "html.element.menuitem.obsolete",
      "message": "The “menuitem” element is obsolete. Use script to handle “contextmenu” event instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 741,
        "byte_start": 718,
        "col": 5,
        "line": 25
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “menuitem” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 780,
        "byte_start": 757,
        "col": 5,
        "line": 26
      }
    },
    {
      "category": "Html",
      "code": "html.element.menuitem.obsolete",
      "message": "The “menuitem” element is obsolete. Use script to handle “contextmenu” event instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 780,
        "byte_start": 757,
        "col": 5,
        "line": 26
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “menuitem” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 819,
        "byte_start": 796,
        "col": 5,
        "line": 27
      }
    },
    {
      "category": "Html",
      "code": "html.element.menuitem.obsolete",
      "message": "The “menuitem” element is obsolete. Use script to handle “contextmenu” event instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 819,
        "byte_start": 796,
        "col": 5,
        "line": 27
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “menuitem” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 928,
        "byte_start": 881,
        "col": 5,
        "line": 30
      }
    },
    {
      "category": "Html",
      "code": "html.element.menuitem.obsolete",
      "message": "The “menuitem” element is obsolete. Use script to handle “contextmenu” event instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 928,
        "byte_start": 881,
        "col": 5,
        "line": 30
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
  "source_name": "html/semantics/menu/tentative/checkable.html"
}
```
