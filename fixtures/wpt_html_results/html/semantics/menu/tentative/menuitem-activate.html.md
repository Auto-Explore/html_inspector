# html/semantics/menu/tentative/menuitem-activate.html

Counts:
- errors: 0
- warnings: 37
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/menu/tentative/menuitem-activate.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta name="timeout" content="long">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-actions.js"></script>
<script src="/resources/testdriver-vendor.js"></script>
<script src="../../popovers/resources/popover-utils.js"></script>
<link rel=author href=mailto:dom@chromium.org>
<link rel=help href=https://open-ui.org/components/menu.explainer>
<style>

menuitem:open { background-color: rgb(12,19,8); }

</style>

<menubar>
 <menuitem id=openMainMenu commandfor=mainmenu command=toggle-menu>Open first menu</menuitem>
</menubar>

<menulist id=mainmenu>
 <menuitem id=openSubMenu commandfor=submenu command=toggle-menu>Toggle menu</menuitem>
 <menuitem id=showPopoverMenuItem command=toggle-popover commandfor=popover>Show popover</menuitem>
 <menuitem id=doNothing>Normal item</menuitem>
</menulist>

<menulist id=submenu>
 <menuitem>Sub menu</menuitem>
</menulist>

<menubar>
  <menuitem id=toggleSubMenu1 commandfor=submenu command=toggle-menu>Toggle menu</menuitem>
  <menuitem id=toggleSubMenu2 commandfor=submenu command=toggle-menu>Toggle menu</menuitem>
  <menuitem id=showSubMenu commandfor=submenu command=show-menu>Show menu</menuitem>
</menubar>

<div popover id=popover>Popover</div>

<button popovertarget=popoverwithmenu>Open popover with menu</button>
<div popover id=popoverwithmenu>
  <button popovertarget=menuinpopover>Open menu in the popover</button>
  <menulist id=menuinpopover>
    <menuitem id=menuinpopoveritem1>First item</menuitem>
    <menuitem id=menuinpopoveritem2 command=toggle-menu commandfor=menuinpopover2>Toggle menu 2</menuitem>
  </menulist>
  <menulist id=menuinpopover2>
    <menuitem>Submenu item</menuitem>
  </menulist>
</div>

<style>
  [popover] {
    width: 400px;
    height: 400px;
  }
  menuitem {
    interest-delay: 1000s;
  }
</style>

<script>

function bgcolor_is_open(element) {
  return getComputedStyle(element).backgroundColor == "rgb(12, 19, 8)";
}

promise_test(async () => {
  assert_false(mainmenu.matches(":popover-open"), "mainmenu popover starts closed");
  assert_false(openMainMenu.matches(":open"), "openMainMenu item starts closed");
  assert_false(bgcolor_is_open(openMainMenu), "openMainMenu item starts closed (style)");
  await clickOn(openMainMenu);
  assert_true(mainmenu.matches(":popover-open"), "mainmenu opens");
  assert_true(openMainMenu.matches(":open"), "openMainMenu item opens");
  assert_true(bgcolor_is_open(openMainMenu), "openMainMenu item opens (style)");

  assert_false(submenu.matches(":popover-open"), "submenu popover starts closed");
  assert_false(openSubMenu.matches(":open"), "openSubMenu item starts closed");
  await clickOn(openSubMenu);
  assert_true(submenu.matches(":popover-open"), "submenu opens");
  assert_true(openSubMenu.matches(":open"), "openSubMenu item opens");

  // Close the submenu.
  await clickOn(openSubMenu);

  assert_false(submenu.matches(":popover-open"), "submenu popover gets closed");
  assert_false(openSubMenu.matches(":open"), "openSubMenu item gets closed");
  assert_true(mainmenu.matches(":popover-open"), "mainmenu still open");

  // Close the mainmenu.
  await clickOn(openMainMenu);
  assert_false(mainmenu.matches(":popover-open"), "mainmenu gets closed");
  assert_false(openMainMenu.matches(":open"), "openMainMenu item gets closed");
}, 'User menuitem activation works with the toggle-menu command');

promise_test(async () => {
  assert_false(mainmenu.matches(":popover-open"), "mainmenu popover starts closed");
  assert_false(openMainMenu.matches(":open"), "openMainMenu item starts closed");
  assert_false(bgcolor_is_open(openMainMenu), "openMainMenu item starts closed (style)");
  await clickOn(openMainMenu);
  assert_true(mainmenu.matches(":popover-open"), "mainmenu popover opens");
  assert_true(openMainMenu.matches(":open"), "openMainMenu item opens");
  assert_true(bgcolor_is_open(openMainMenu), "openMainMenu item opens (style)");

  assert_false(popover.matches(":popover-open"), "div popover starts closed");
  assert_false(showPopoverMenuItem.matches(":open"), "showPopoverMenuItem item starts closed");
  await clickOn(showPopoverMenuItem);
  assert_true(popover.matches(":popover-open"), "div popover opens");
  assert_false(mainmenu.matches(":popover-open"), "mainmenu popover closes");
  assert_false(showPopoverMenuItem.matches(":open"), "showPopoverMenuItem item stays closed");

  // Close the popover.
  await clickOn(openMainMenu);
  assert_false(popover.matches(":popover-open"), "div popover gets closed");
  assert_true(mainmenu.matches(":popover-open"), "mainmenu gets opened");
  await clickOn(openMainMenu);
  assert_false(mainmenu.matches(":popover-open"), "mainmenu gets closed");
}, 'User menuitem activation works with show-popover command');

promise_test(async (t) => {
  assert_false(popoverwithmenu.matches(":popover-open"),
      "popover with menu starts closed");

  // Open the popover that hosts two menulists.
  await clickOn(
      document.querySelector("button[popovertarget=popoverwithmenu]"));
  assert_true(popoverwithmenu.matches(":popover-open"),
      "popover with menu opens");
  assert_false(menuinpopover.matches(":popover-open"),
      "menu in popover starts closed");

  // Open the first menu in the popover.
  await clickOn(
      document.querySelector('button[popovertarget=menuinpopover]'));
  assert_true(menuinpopover.matches(":popover-open"), "menu in popover opens");
  assert_true(popoverwithmenu.matches(":popover-open"),
      "outer popover remains open");
  assert_false(menuinpopover2.matches(":popover-open"),
      "menu 2 in popover starts closed");

  // Open the second menu in the popover.
  await clickOn(menuinpopoveritem2);
  assert_true(menuinpopover2.matches(":popover-open"),
      "menu 2 in popover opens");
  assert_true(popoverwithmenu.matches(":popover-open"),
      "outer popover remains open after opening menu 2");
  assert_true(menuinpopover.matches(":popover-open"),
      "menu in popover remains open");

  // Close the second, "sub", menu within the popover by just clicking off of
  // it.
  await clickOn(menuinpopoveritem2);
  assert_false(menuinpopover2.matches(":popover-open"),
      "menu 2 in popover closes");
  assert_true(popoverwithmenu.matches(":popover-open"),
      "outer popover remains open after closing menu 2");
  assert_true(menuinpopover.matches(":popover-open"),
      "menu in popover remains open");
}, 'Menulist inside a popover works correctly; does not get accidentally ' +
   'dismissed by opening submenus');

async function getMenuItemCoords(invoker, targetMenuItem) {
  // test_driver isn't suited to mousedown-drag-mouseup interactions when the
  // mousedown triggers visibility of one of the elements.
  const menulist = targetMenuItem.parentElement;
  assert_false(menulist.matches(":popover-open"), "menulist popover should start closed");
  await clickOn(invoker);
  assert_true(menulist.matches(":popover-open"), "menulist popover opens when clicked");
  let rect = targetMenuItem.getBoundingClientRect();
  let coords = {x: Math.round(rect.x + rect.width / 2),
      y: Math.round(rect.y + rect.height / 2)};
  await clickOn(invoker);
  assert_false(menulist.matches(":popover-open"), "menulist popover closes when clicked");
  return coords;
}

promise_test(async (t) => {
  assert_false(mainmenu.matches(":popover-open"), "mainmenu popover starts closed");
  const doNothingCoordinates = await getMenuItemCoords(openMainMenu, doNothing);
  let invokerClicks = 0;
  let itemClicks = 0;
  openMainMenu.addEventListener('click',() => (++invokerClicks));
  doNothing.addEventListener('click',() => (++itemClicks));
  let openStateAfterPointerdown = "none";
  let gotPointerMove = new Promise(resolve => {
    openMainMenu.addEventListener('pointermove',async () => {
      // There will be two move events, one before the pointerdown and one after.
      // Just capture the one after.
      if (openStateAfterPointerdown === "none") {
        openStateAfterPointerdown = "first-move";
      } else if (openStateAfterPointerdown === "first-move") {
        await waitForRender();
        openStateAfterPointerdown = mainmenu.matches(":popover-open") ? "open" : "closed";
        resolve();
      }
    },{signal: t.get_signal()});
  })
  await new test_driver.Actions()
    .addPointer('mouse', 'mouse')
    .pointerMove(0, 0, {origin: openMainMenu})
    .pointerDown()
    // Extra move to trigger event on openMainMenu:
    .pointerMove(2, 2, {origin: openMainMenu})
    .pointerMove(doNothingCoordinates.x, doNothingCoordinates.y, {})
    .pointerUp()
    .send();
  await gotPointerMove;
  assert_equals(openStateAfterPointerdown,"open", "mainmenu popover should open after pointer down");
  assert_false(mainmenu.matches(":popover-open"), "mainmenu popover should be closed after interaction");
  assert_equals(invokerClicks,0, "the invoking menu didn't get a click");
  // TODO: Menu items should fire a click event when they are selected.
  // assert_equals(itemClicks,1, "the invoked menu did get a click");
}, 'A mousedown-drag-mouseup gesture on a normal menuitem picks the item');

promise_test(async (t) => {
  assert_false(mainmenu.matches(":popover-open"), "mainmenu popover starts closed");
  assert_false(submenu.matches(":popover-open"), "submenu popover starts closed");
  const openSubMenuCoordinates = await getMenuItemCoords(openMainMenu, openSubMenu);
  await new test_driver.Actions()
    .addPointer('mouse', 'mouse')
    .pointerMove(0, 0, {origin: openMainMenu})
    .pointerDown()
    .pointerMove(openSubMenuCoordinates.x, openSubMenuCoordinates.y, {})
    .pointerUp()
    .send();
  await waitForRender();
  assert_true(mainmenu.matches(":popover-open"), "mainmenu popover should remain open, because submenu chosen");
  assert_true(submenu.matches(":popover-open"), "submenu popover should be open");
  await clickOn(openMainMenu); // Cleanup.
  assert_false(mainmenu.matches(":popover-open"), "mainmenu popover should be closed");
  assert_false(submenu.matches(":popover-open"), "submenu popover should be closed");
}, 'A mousedown-drag-mouseup gesture on a submenu item leaves both menus open');

promise_test(async () => {
  assert_false(submenu.matches(":popover-open"), "submenu popover starts closed");
  assert_false(openMainMenu.matches(":open"), "openMainMenu item starts closed");
  assert_false(bgcolor_is_open(openMainMenu), "openMainMenu item starts closed (style)");
  await clickOn(toggleSubMenu1);
  assert_true(submenu.matches(":popover-open"), "submenu opens");
  assert_true(toggleSubMenu1.matches(":open"), "toggleSubMenu1 item opens");
  assert_true(bgcolor_is_open(toggleSubMenu1), "toggleSubMenu1 item opens (style)");
  assert_false(toggleSubMenu2.matches(":open"), "toggleSubMenu2 item stays closed");
  assert_false(bgcolor_is_open(toggleSubMenu2), "toggleSubMenu2 item stays closed (style)");
  assert_false(showSubMenu.matches(":open"), "showSubMenu item stays closed");
  assert_false(bgcolor_is_open(showSubMenu), "showSubMenu item stays closed (style)");
  // FIXME(https://crbug.com/406566432): Should closing the menu first be needed?
  await clickOn(toggleSubMenu1);
  await clickOn(toggleSubMenu2);
  assert_true(submenu.matches(":popover-open"), "submenu again open after toggle");
  assert_false(toggleSubMenu1.matches(":open"), "toggleSubMenu1 item closes");
  assert_false(bgcolor_is_open(toggleSubMenu1), "toggleSubMenu1 item closes (style)");
  assert_true(toggleSubMenu2.matches(":open"), "toggleSubMenu2 item opens");
  assert_true(bgcolor_is_open(toggleSubMenu2), "toggleSubMenu2 item opens (style)");
  assert_false(showSubMenu.matches(":open"), "showSubMenu item stays closed");
  assert_false(bgcolor_is_open(showSubMenu), "showSubMenu item stays closed (style)");
  // FIXME(https://crbug.com/406566432): Should closing the menu first be needed?
  await clickOn(toggleSubMenu2);
  await clickOn(showSubMenu);
  assert_true(submenu.matches(":popover-open"), "submenu again open after show");
  assert_false(toggleSubMenu1.matches(":open"), "toggleSubMenu1 item stays closed");
  assert_false(bgcolor_is_open(toggleSubMenu1), "toggleSubMenu1 item stays closed (style)");
  assert_false(toggleSubMenu2.matches(":open"), "toggleSubMenu2 item closes");
  assert_false(bgcolor_is_open(toggleSubMenu2), "toggleSubMenu2 item closes (style)");
  assert_false(showSubMenu.matches(":open"), "showSubMenu item stays closed since :open does not apply");
  assert_false(bgcolor_is_open(showSubMenu), "showSubMenu item stays closed since :open does not apply (style)");
}, ':open only applies to the invoker and only works for toggle-menu commands');
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
        "byte_end": 580,
        "byte_start": 571,
        "col": 1,
        "line": 17
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “menubar” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 580,
        "byte_start": 571,
        "col": 1,
        "line": 17
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “menuitem” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 648,
        "byte_start": 582,
        "col": 2,
        "line": 18
      }
    },
    {
      "category": "Html",
      "code": "html.element.menuitem.obsolete",
      "message": "The “menuitem” element is obsolete. Use script to handle “contextmenu” event instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 648,
        "byte_start": 582,
        "col": 2,
        "line": 18
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “menulist” not allowed as child of “body” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 709,
        "byte_start": 687,
        "col": 1,
        "line": 21
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “menulist” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 709,
        "byte_start": 687,
        "col": 1,
        "line": 21
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “menuitem” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 775,
        "byte_start": 711,
        "col": 2,
        "line": 22
      }
    },
    {
      "category": "Html",
      "code": "html.element.menuitem.obsolete",
      "message": "The “menuitem” element is obsolete. Use script to handle “contextmenu” event instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 775,
        "byte_start": 711,
        "col": 2,
        "line": 22
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “menuitem” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 874,
        "byte_start": 799,
        "col": 2,
        "line": 23
      }
    },
    {
      "category": "Html",
      "code": "html.element.menuitem.obsolete",
      "message": "The “menuitem” element is obsolete. Use script to handle “contextmenu” event instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 874,
        "byte_start": 799,
        "col": 2,
        "line": 23
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “menuitem” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 922,
        "byte_start": 899,
        "col": 2,
        "line": 24
      }
    },
    {
      "category": "Html",
      "code": "html.element.menuitem.obsolete",
      "message": "The “menuitem” element is obsolete. Use script to handle “contextmenu” event instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 922,
        "byte_start": 899,
        "col": 2,
        "line": 24
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “menulist” not allowed as child of “body” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 979,
        "byte_start": 958,
        "col": 1,
        "line": 27
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “menulist” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 979,
        "byte_start": 958,
        "col": 1,
        "line": 27
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “menuitem” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 991,
        "byte_start": 981,
        "col": 2,
        "line": 28
      }
    },
    {
      "category": "Html",
      "code": "html.element.menuitem.obsolete",
      "message": "The “menuitem” element is obsolete. Use script to handle “contextmenu” event instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 991,
        "byte_start": 981,
        "col": 2,
        "line": 28
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “menubar” not allowed as child of “body” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 1033,
        "byte_start": 1024,
        "col": 1,
        "line": 31
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “menubar” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 1033,
        "byte_start": 1024,
        "col": 1,
        "line": 31
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “menuitem” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 1103,
        "byte_start": 1036,
        "col": 3,
        "line": 32
      }
    },
    {
      "category": "Html",
      "code": "html.element.menuitem.obsolete",
      "message": "The “menuitem” element is obsolete. Use script to handle “contextmenu” event instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 1103,
        "byte_start": 1036,
        "col": 3,
        "line": 32
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “menuitem” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 1195,
        "byte_start": 1128,
        "col": 3,
        "line": 33
      }
    },
    {
      "category": "Html",
      "code": "html.element.menuitem.obsolete",
      "message": "The “menuitem” element is obsolete. Use script to handle “contextmenu” event instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 1195,
        "byte_start": 1128,
        "col": 3,
        "line": 33
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “menuitem” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 1282,
        "byte_start": 1220,
        "col": 3,
        "line": 34
      }
    },
    {
      "category": "Html",
      "code": "html.element.menuitem.obsolete",
      "message": "The “menuitem” element is obsolete. Use script to handle “contextmenu” event instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 1282,
        "byte_start": 1220,
        "col": 3,
        "line": 34
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “menulist” not allowed as child of “div” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 1558,
        "byte_start": 1531,
        "col": 3,
        "line": 42
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “menulist” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 1558,
        "byte_start": 1531,
        "col": 3,
        "line": 42
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “menuitem” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 1595,
        "byte_start": 1563,
        "col": 5,
        "line": 43
      }
    },
    {
      "category": "Html",
      "code": "html.element.menuitem.obsolete",
      "message": "The “menuitem” element is obsolete. Use script to handle “contextmenu” event instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 1595,
        "byte_start": 1563,
        "col": 5,
        "line": 43
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “menuitem” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 1699,
        "byte_start": 1621,
        "col": 5,
        "line": 44
      }
    },
    {
      "category": "Html",
      "code": "html.element.menuitem.obsolete",
      "message": "The “menuitem” element is obsolete. Use script to handle “contextmenu” event instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 1699,
        "byte_start": 1621,
        "col": 5,
        "line": 44
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “menulist” not allowed as child of “div” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 1768,
        "byte_start": 1740,
        "col": 3,
        "line": 46
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “menulist” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 1768,
        "byte_start": 1740,
        "col": 3,
        "line": 46
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “menuitem” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 1783,
        "byte_start": 1773,
        "col": 5,
        "line": 47
      }
    },
    {
      "category": "Html",
      "code": "html.element.menuitem.obsolete",
      "message": "The “menuitem” element is obsolete. Use script to handle “contextmenu” event instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 1783,
        "byte_start": 1773,
        "col": 5,
        "line": 47
      }
    },
    {
      "category": "Html",
      "code": "html.style.not_allowed_here",
      "message": "Element “style” not allowed as child of “body” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 1836,
        "byte_start": 1829,
        "col": 1,
        "line": 51
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
  "source_name": "html/semantics/menu/tentative/menuitem-activate.html"
}
```
