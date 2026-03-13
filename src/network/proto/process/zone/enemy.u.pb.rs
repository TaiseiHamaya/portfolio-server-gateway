const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.32.1-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut Proto__PayloadEnemySpawn_msg_init: ::protobuf::__internal::runtime::MiniTablePtr =
    ::protobuf::__internal::runtime::MiniTablePtr(::std::ptr::null_mut());
#[allow(non_camel_case_types)]
pub struct PayloadEnemySpawn {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<PayloadEnemySpawn>
}

impl ::protobuf::Message for PayloadEnemySpawn {}

impl ::std::default::Default for PayloadEnemySpawn {
  fn default() -> Self {
    Self::new()
  }
}

impl ::protobuf::Parse for PayloadEnemySpawn {
  fn parse(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    Self::parse(serialized)
  }

  fn parse_dont_enforce_required(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    Self::parse_dont_enforce_required(serialized)
  }
}

impl ::std::fmt::Debug for PayloadEnemySpawn {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    let string = unsafe {
      ::protobuf::__internal::runtime::debug_string(
        self.raw_msg(),
        <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table()
      )
    };
    write!(f, "{}", string)
  }
}

impl ::protobuf::Serialize for PayloadEnemySpawn {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

// SAFETY:
// - `PayloadEnemySpawn` is `Sync` because it does not implement interior mutability.
//    Neither does `PayloadEnemySpawnMut`.
unsafe impl Sync for PayloadEnemySpawn {}

// SAFETY:
// - `PayloadEnemySpawn` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for PayloadEnemySpawn {}

impl ::protobuf::Proxied for PayloadEnemySpawn {
  type View<'msg> = PayloadEnemySpawnView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for PayloadEnemySpawn {}

impl ::protobuf::MutProxied for PayloadEnemySpawn {
  type Mut<'msg> = PayloadEnemySpawnMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct PayloadEnemySpawnView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, PayloadEnemySpawn>,
  _phantom: ::std::marker::PhantomData<&'msg ()>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PayloadEnemySpawnView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for PayloadEnemySpawnView<'msg> {
  type Message = PayloadEnemySpawn;
}

impl ::std::fmt::Debug for PayloadEnemySpawnView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    let string = unsafe {
      ::protobuf::__internal::runtime::debug_string(
        self.raw_msg(),
        <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table()
      )
    };
    write!(f, "{}", string)
  }
}

impl ::protobuf::Serialize for PayloadEnemySpawnView<'_> {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    // SAFETY: `MINI_TABLE` is the one associated with `self.raw_msg()`.
    let encoded = unsafe {
      ::protobuf::__internal::runtime::wire::encode(self.raw_msg(),
          <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table())
    };
    encoded.map_err(|_| ::protobuf::SerializeError)
  }
}

impl ::std::default::Default for PayloadEnemySpawnView<'_> {
  fn default() -> PayloadEnemySpawnView<'static> {
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(::protobuf::__internal::runtime::ScratchSpace::zeroed_block()) };
    PayloadEnemySpawnView::new(::protobuf::__internal::Private, inner)
  }
}

#[allow(dead_code)]
impl<'msg> PayloadEnemySpawnView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::protobuf::__internal::Private, inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, PayloadEnemySpawn>) -> Self {
    Self { inner, _phantom: ::std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  pub fn to_owned(&self) -> PayloadEnemySpawn {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // id: optional uint64
  pub fn id(self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        0, (0u64).into()
      ).try_into().unwrap()
    }
  }

  // name: optional string
  pub fn name(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // position: optional message Proto.Vector3
  pub fn has_position(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn position_opt(self) -> ::protobuf::Optional<super::Vector3View<'msg>> {
        ::protobuf::Optional::new(self.position(), self.has_position())
  }
  pub fn position(self) -> super::Vector3View<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    let raw = submsg.map(|ptr| ptr.raw()).unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    super::Vector3View::new(::protobuf::__internal::Private, inner)
  }

}

// SAFETY:
// - `PayloadEnemySpawnView` is `Sync` because it does not support mutation.
unsafe impl Sync for PayloadEnemySpawnView<'_> {}

// SAFETY:
// - `PayloadEnemySpawnView` is `Send` because while its alive a `PayloadEnemySpawnMut` cannot.
// - `PayloadEnemySpawnView` does not use thread-local data.
unsafe impl Send for PayloadEnemySpawnView<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for PayloadEnemySpawnView<'msg> {}
impl<'msg> ::protobuf::ViewProxy<'msg> for PayloadEnemySpawnView<'msg> {}

impl<'msg> ::protobuf::AsView for PayloadEnemySpawnView<'msg> {
  type Proxied = PayloadEnemySpawn;
  fn as_view(&self) -> ::protobuf::View<'msg, PayloadEnemySpawn> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PayloadEnemySpawnView<'msg> {
  fn into_view<'shorter>(self) -> PayloadEnemySpawnView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<PayloadEnemySpawn> for PayloadEnemySpawnView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> PayloadEnemySpawn {
    let mut dst = PayloadEnemySpawn::new();
    let dst_raw = ::protobuf::__internal::runtime::UpbGetMessagePtrMut::get_raw_message_mut(&mut dst, ::protobuf::__internal::Private);
    let dst_arena = ::protobuf::__internal::runtime::UpbGetArena::get_arena(&mut dst, ::protobuf::__internal::Private);
    let src_raw = ::protobuf::__internal::runtime::UpbGetMessagePtr::get_raw_message(&self, ::protobuf::__internal::Private);

    unsafe { ::protobuf::__internal::runtime::upb_Message_DeepCopy(
      dst_raw,
      src_raw,
      <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
      dst_arena.raw(),
    ) };
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<PayloadEnemySpawn> for PayloadEnemySpawnMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> PayloadEnemySpawn {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::UpbTypeConversions for PayloadEnemySpawn {
    fn upb_type() -> ::protobuf::__internal::runtime::CType {
        ::protobuf::__internal::runtime::CType::Message
    }

    fn to_message_value(
        val: ::protobuf::View<'_, Self>) -> ::protobuf::__internal::runtime::upb_MessageValue {
        ::protobuf::__internal::runtime::upb_MessageValue { msg_val: Some(val.raw_msg()) }
    }

    unsafe fn into_message_value_fuse_if_required(
      raw_parent_arena: ::protobuf::__internal::runtime::RawArena,
      mut val: Self) -> ::protobuf::__internal::runtime::upb_MessageValue {
      // SAFETY: The arena memory is not freed due to `ManuallyDrop`.
      let parent_arena = ::std::mem::ManuallyDrop::new(
          unsafe { ::protobuf::__internal::runtime::Arena::from_raw(raw_parent_arena) });

      parent_arena.fuse(val.as_message_mut_inner(::protobuf::__internal::Private).arena());
      ::protobuf::__internal::runtime::upb_MessageValue { msg_val: Some(val.raw_msg()) }
    }

    unsafe fn from_message_value<'msg>(msg: ::protobuf::__internal::runtime::upb_MessageValue)
        -> ::protobuf::View<'msg, Self> {
        let raw = unsafe { msg.msg_val }.expect("expected present message value in map");
        let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
        PayloadEnemySpawnView::new(::protobuf::__internal::Private, inner)
    }

    unsafe fn from_message_mut<'msg>(msg: ::protobuf::__internal::runtime::RawMessage, arena: &'msg ::protobuf::__internal::runtime::Arena)
        -> PayloadEnemySpawnMut<'msg> {
        let inner = unsafe { ::protobuf::__internal::runtime::MessageMutInner::<'msg, PayloadEnemySpawn>::wrap_raw(msg, arena) };
        PayloadEnemySpawnMut::new(::protobuf::__internal::Private, inner)
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct PayloadEnemySpawnMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, PayloadEnemySpawn>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PayloadEnemySpawnMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for PayloadEnemySpawnMut<'msg> {
  type Message = PayloadEnemySpawn;
}

impl ::std::fmt::Debug for PayloadEnemySpawnMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    let string = unsafe {
      ::protobuf::__internal::runtime::debug_string(
        self.raw_msg(),
        <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table()
      )
    };
    write!(f, "{}", string)
  }
}

impl ::protobuf::Serialize for PayloadEnemySpawnMut<'_> {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

#[allow(dead_code)]
impl<'msg> PayloadEnemySpawnMut<'msg> {
  #[doc(hidden)]
  pub fn from_parent<ParentT: ::protobuf::Message>(
             _private: ::protobuf::__internal::Private,
             parent: ::protobuf::__internal::runtime::MessageMutInner<'msg, ParentT>,
             msg: ::protobuf::__internal::runtime::RawMessage)
    -> Self {
    Self {
      inner: ::protobuf::__internal::runtime::MessageMutInner::from_parent(parent, msg)
    }
  }

  #[doc(hidden)]
  pub fn new(_private: ::protobuf::__internal::Private, inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, PayloadEnemySpawn>) -> Self {
    Self { inner }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, PayloadEnemySpawn> {
    self.inner
  }

  pub fn to_owned(&self) -> PayloadEnemySpawn {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  fn arena(&mut self) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }

  // id: optional uint64
  pub fn id(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        0, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_id(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        0, val.into()
      )
    }
  }

  // name: optional string
  pub fn name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    let (view, arena) =
      s.into_inner(::protobuf::__internal::Private).into_raw_parts();

    let parent_arena = self.inner.arena();
    parent_arena.fuse(&arena);

    unsafe {
      self.inner.ptr_mut().set_base_field_string_at_index(
        1,
        view,
      );
    }
  }

  // position: optional message Proto.Vector3
  pub fn has_position(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_position(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn position_opt(&self) -> ::protobuf::Optional<super::Vector3View<'_>> {
        ::protobuf::Optional::new(self.position(), self.has_position())
  }
  pub fn position(&self) -> super::Vector3View<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    let raw = submsg.map(|ptr| ptr.raw()).unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    super::Vector3View::new(::protobuf::__internal::Private, inner)
  }
  pub fn position_mut(&mut self) -> super::Vector3Mut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         2, self.arena()
       ).unwrap()
     };
     super::Vector3Mut::from_parent(
       ::protobuf::__internal::Private,
       self.as_message_mut_inner(::protobuf::__internal::Private),
       ptr.raw())
  }
  pub fn set_position(&mut self,
    val: impl ::protobuf::IntoProxied<super::Vector3>) {

    // The message and arena are dropped after the setter. The
    // memory remains allocated as we fuse the arena with the
    // parent message's arena.
    let mut child = val.into_proxied(::protobuf::__internal::Private);
    self.inner
      .arena()
      .fuse(::protobuf::__internal::runtime::UpbGetArena::get_arena(&mut child, ::protobuf::__internal::Private));

    let child_ptr = ::protobuf::__internal::runtime::UpbGetMessagePtrMut::get_ptr_mut(&mut child, ::protobuf::__internal::Private);
    unsafe {
      self.inner.ptr_mut().set_base_field_message_at_index(
        2, child_ptr
      );
    }
  }

}

// SAFETY:
// - `PayloadEnemySpawnMut` does not perform any shared mutation.
// - `PayloadEnemySpawnMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for PayloadEnemySpawnMut<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for PayloadEnemySpawnMut<'msg> {}
impl<'msg> ::protobuf::MutProxy<'msg> for PayloadEnemySpawnMut<'msg> {}

impl<'msg> ::protobuf::AsView for PayloadEnemySpawnMut<'msg> {
  type Proxied = PayloadEnemySpawn;
  fn as_view(&self) -> ::protobuf::View<'_, PayloadEnemySpawn> {
    PayloadEnemySpawnView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner.clone()),
      _phantom: ::std::marker::PhantomData
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PayloadEnemySpawnMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, PayloadEnemySpawn>
  where
      'msg: 'shorter {
    PayloadEnemySpawnView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner.clone()),
      _phantom: ::std::marker::PhantomData
    }
  }
}

impl<'msg> ::protobuf::AsMut for PayloadEnemySpawnMut<'msg> {
  type MutProxied = PayloadEnemySpawn;
  fn as_mut(&mut self) -> PayloadEnemySpawnMut<'msg> {
    PayloadEnemySpawnMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for PayloadEnemySpawnMut<'msg> {
  fn into_mut<'shorter>(self) -> PayloadEnemySpawnMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl PayloadEnemySpawn {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, PayloadEnemySpawn> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  fn arena(&mut self) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }

  pub fn parse(data: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    let mut msg = Self::new();
    ::protobuf::ClearAndParse::clear_and_parse(&mut msg, data).map(|_| msg)
  }

  pub fn parse_dont_enforce_required(data: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    let mut msg = Self::new();
    ::protobuf::ClearAndParse::clear_and_parse_dont_enforce_required(&mut msg, data).map(|_| msg)
  }

  pub fn as_view(&self) -> PayloadEnemySpawnView {
    PayloadEnemySpawnView::new(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner))
  }

  pub fn as_mut(&mut self) -> PayloadEnemySpawnMut {
    let inner = ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner);
    PayloadEnemySpawnMut::new(::protobuf::__internal::Private, inner)
  }

  // id: optional uint64
  pub fn id(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        0, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_id(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        0, val.into()
      )
    }
  }

  // name: optional string
  pub fn name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    let (view, arena) =
      s.into_inner(::protobuf::__internal::Private).into_raw_parts();

    let parent_arena = self.inner.arena();
    parent_arena.fuse(&arena);

    unsafe {
      self.inner.ptr_mut().set_base_field_string_at_index(
        1,
        view,
      );
    }
  }

  // position: optional message Proto.Vector3
  pub fn has_position(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_position(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn position_opt(&self) -> ::protobuf::Optional<super::Vector3View<'_>> {
        ::protobuf::Optional::new(self.position(), self.has_position())
  }
  pub fn position(&self) -> super::Vector3View<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    let raw = submsg.map(|ptr| ptr.raw()).unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    super::Vector3View::new(::protobuf::__internal::Private, inner)
  }
  pub fn position_mut(&mut self) -> super::Vector3Mut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         2, self.arena()
       ).unwrap()
     };
     super::Vector3Mut::from_parent(
       ::protobuf::__internal::Private,
       self.as_message_mut_inner(::protobuf::__internal::Private),
       ptr.raw())
  }
  pub fn set_position(&mut self,
    val: impl ::protobuf::IntoProxied<super::Vector3>) {

    // The message and arena are dropped after the setter. The
    // memory remains allocated as we fuse the arena with the
    // parent message's arena.
    let mut child = val.into_proxied(::protobuf::__internal::Private);
    self.inner
      .arena()
      .fuse(::protobuf::__internal::runtime::UpbGetArena::get_arena(&mut child, ::protobuf::__internal::Private));

    let child_ptr = ::protobuf::__internal::runtime::UpbGetMessagePtrMut::get_ptr_mut(&mut child, ::protobuf::__internal::Private);
    unsafe {
      self.inner.ptr_mut().set_base_field_message_at_index(
        2, child_ptr
      );
    }
  }

}  // impl PayloadEnemySpawn

impl ::std::ops::Drop for PayloadEnemySpawn {
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for PayloadEnemySpawn {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for PayloadEnemySpawn {
  type Proxied = Self;
  fn as_view(&self) -> PayloadEnemySpawnView {
    self.as_view()
  }
}

impl ::protobuf::AsMut for PayloadEnemySpawn {
  type MutProxied = Self;
  fn as_mut(&mut self) -> PayloadEnemySpawnMut {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for PayloadEnemySpawn {
  fn mini_table() -> *const ::protobuf::__internal::runtime::upb_MiniTable {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTablePtr> =
        ::std::sync::OnceLock::new();
    ONCE_LOCK.get_or_init(|| unsafe {
      super::Proto__PayloadEnemySpawn_msg_init.0 =
          ::protobuf::__internal::runtime::upb_MiniTable_Build(
              "$,P1X3".as_ptr(),
              6,
              ::protobuf::__internal::runtime::THREAD_LOCAL_ARENA.with(|a| a.raw()),
              ::std::ptr::null_mut());
      let submessages = [
        <super::Vector3 as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
      ];
      let subenums = [
      ];
      assert!(::protobuf::__internal::runtime::upb_MiniTable_Link(
          super::Proto__PayloadEnemySpawn_msg_init.0,
          submessages.as_ptr() as *const *const ::protobuf::__internal::runtime::upb_MiniTable,
          submessages.len(), subenums.as_ptr(), subenums.len()));
      ::protobuf::__internal::runtime::MiniTablePtr(super::Proto__PayloadEnemySpawn_msg_init.0)
    }).0
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for PayloadEnemySpawn {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for PayloadEnemySpawnView<'_> {
  #[inline(always)]
  fn mini_table() -> *const ::protobuf::__internal::runtime::upb_MiniTable {
    <PayloadEnemySpawn as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for PayloadEnemySpawnMut<'_> {
  #[inline(always)]
  fn mini_table() -> *const ::protobuf::__internal::runtime::upb_MiniTable {
    <PayloadEnemySpawn as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for PayloadEnemySpawn {
  type Msg = PayloadEnemySpawn;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PayloadEnemySpawn> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PayloadEnemySpawn {
  type Msg = PayloadEnemySpawn;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PayloadEnemySpawn> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for PayloadEnemySpawnMut<'_> {
  type Msg = PayloadEnemySpawn;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PayloadEnemySpawn> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PayloadEnemySpawnMut<'_> {
  type Msg = PayloadEnemySpawn;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PayloadEnemySpawn> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PayloadEnemySpawnView<'_> {
  type Msg = PayloadEnemySpawn;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PayloadEnemySpawn> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for PayloadEnemySpawnMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}


// upb kernel doesn't support any owned message or message mut interop.
impl ::protobuf::OwnedMessageInterop for PayloadEnemySpawn {}
impl<'a> ::protobuf::MessageMutInterop<'a> for PayloadEnemySpawnMut<'a> {}

impl<'a> ::protobuf::MessageViewInterop<'a> for PayloadEnemySpawnView<'a> {
  unsafe fn __unstable_wrap_raw_message(
    msg: &'a *const ::std::ffi::c_void) -> Self {
    let raw = ::protobuf::__internal::runtime::RawMessage::new(*msg as *mut _).unwrap();
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    Self::new(::protobuf::__internal::Private, inner)
  }
  unsafe fn __unstable_wrap_raw_message_unchecked_lifetime(
    msg: *const ::std::ffi::c_void) -> Self {
    let raw = ::protobuf::__internal::runtime::RawMessage::new(msg as *mut _).unwrap();
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    Self::new(::protobuf::__internal::Private, inner)
  }
  fn __unstable_as_raw_message(&self) -> *const ::std::ffi::c_void {
    self.inner.raw().as_ptr() as *const _
  }
}

// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut Proto__PayloadEnemyDespawn_msg_init: ::protobuf::__internal::runtime::MiniTablePtr =
    ::protobuf::__internal::runtime::MiniTablePtr(::std::ptr::null_mut());
#[allow(non_camel_case_types)]
pub struct PayloadEnemyDespawn {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<PayloadEnemyDespawn>
}

impl ::protobuf::Message for PayloadEnemyDespawn {}

impl ::std::default::Default for PayloadEnemyDespawn {
  fn default() -> Self {
    Self::new()
  }
}

impl ::protobuf::Parse for PayloadEnemyDespawn {
  fn parse(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    Self::parse(serialized)
  }

  fn parse_dont_enforce_required(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    Self::parse_dont_enforce_required(serialized)
  }
}

impl ::std::fmt::Debug for PayloadEnemyDespawn {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    let string = unsafe {
      ::protobuf::__internal::runtime::debug_string(
        self.raw_msg(),
        <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table()
      )
    };
    write!(f, "{}", string)
  }
}

impl ::protobuf::Serialize for PayloadEnemyDespawn {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

// SAFETY:
// - `PayloadEnemyDespawn` is `Sync` because it does not implement interior mutability.
//    Neither does `PayloadEnemyDespawnMut`.
unsafe impl Sync for PayloadEnemyDespawn {}

// SAFETY:
// - `PayloadEnemyDespawn` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for PayloadEnemyDespawn {}

impl ::protobuf::Proxied for PayloadEnemyDespawn {
  type View<'msg> = PayloadEnemyDespawnView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for PayloadEnemyDespawn {}

impl ::protobuf::MutProxied for PayloadEnemyDespawn {
  type Mut<'msg> = PayloadEnemyDespawnMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct PayloadEnemyDespawnView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, PayloadEnemyDespawn>,
  _phantom: ::std::marker::PhantomData<&'msg ()>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PayloadEnemyDespawnView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for PayloadEnemyDespawnView<'msg> {
  type Message = PayloadEnemyDespawn;
}

impl ::std::fmt::Debug for PayloadEnemyDespawnView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    let string = unsafe {
      ::protobuf::__internal::runtime::debug_string(
        self.raw_msg(),
        <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table()
      )
    };
    write!(f, "{}", string)
  }
}

impl ::protobuf::Serialize for PayloadEnemyDespawnView<'_> {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    // SAFETY: `MINI_TABLE` is the one associated with `self.raw_msg()`.
    let encoded = unsafe {
      ::protobuf::__internal::runtime::wire::encode(self.raw_msg(),
          <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table())
    };
    encoded.map_err(|_| ::protobuf::SerializeError)
  }
}

impl ::std::default::Default for PayloadEnemyDespawnView<'_> {
  fn default() -> PayloadEnemyDespawnView<'static> {
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(::protobuf::__internal::runtime::ScratchSpace::zeroed_block()) };
    PayloadEnemyDespawnView::new(::protobuf::__internal::Private, inner)
  }
}

#[allow(dead_code)]
impl<'msg> PayloadEnemyDespawnView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::protobuf::__internal::Private, inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, PayloadEnemyDespawn>) -> Self {
    Self { inner, _phantom: ::std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  pub fn to_owned(&self) -> PayloadEnemyDespawn {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // id: optional uint64
  pub fn id(self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        0, (0u64).into()
      ).try_into().unwrap()
    }
  }

}

// SAFETY:
// - `PayloadEnemyDespawnView` is `Sync` because it does not support mutation.
unsafe impl Sync for PayloadEnemyDespawnView<'_> {}

// SAFETY:
// - `PayloadEnemyDespawnView` is `Send` because while its alive a `PayloadEnemyDespawnMut` cannot.
// - `PayloadEnemyDespawnView` does not use thread-local data.
unsafe impl Send for PayloadEnemyDespawnView<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for PayloadEnemyDespawnView<'msg> {}
impl<'msg> ::protobuf::ViewProxy<'msg> for PayloadEnemyDespawnView<'msg> {}

impl<'msg> ::protobuf::AsView for PayloadEnemyDespawnView<'msg> {
  type Proxied = PayloadEnemyDespawn;
  fn as_view(&self) -> ::protobuf::View<'msg, PayloadEnemyDespawn> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PayloadEnemyDespawnView<'msg> {
  fn into_view<'shorter>(self) -> PayloadEnemyDespawnView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<PayloadEnemyDespawn> for PayloadEnemyDespawnView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> PayloadEnemyDespawn {
    let mut dst = PayloadEnemyDespawn::new();
    let dst_raw = ::protobuf::__internal::runtime::UpbGetMessagePtrMut::get_raw_message_mut(&mut dst, ::protobuf::__internal::Private);
    let dst_arena = ::protobuf::__internal::runtime::UpbGetArena::get_arena(&mut dst, ::protobuf::__internal::Private);
    let src_raw = ::protobuf::__internal::runtime::UpbGetMessagePtr::get_raw_message(&self, ::protobuf::__internal::Private);

    unsafe { ::protobuf::__internal::runtime::upb_Message_DeepCopy(
      dst_raw,
      src_raw,
      <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
      dst_arena.raw(),
    ) };
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<PayloadEnemyDespawn> for PayloadEnemyDespawnMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> PayloadEnemyDespawn {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::UpbTypeConversions for PayloadEnemyDespawn {
    fn upb_type() -> ::protobuf::__internal::runtime::CType {
        ::protobuf::__internal::runtime::CType::Message
    }

    fn to_message_value(
        val: ::protobuf::View<'_, Self>) -> ::protobuf::__internal::runtime::upb_MessageValue {
        ::protobuf::__internal::runtime::upb_MessageValue { msg_val: Some(val.raw_msg()) }
    }

    unsafe fn into_message_value_fuse_if_required(
      raw_parent_arena: ::protobuf::__internal::runtime::RawArena,
      mut val: Self) -> ::protobuf::__internal::runtime::upb_MessageValue {
      // SAFETY: The arena memory is not freed due to `ManuallyDrop`.
      let parent_arena = ::std::mem::ManuallyDrop::new(
          unsafe { ::protobuf::__internal::runtime::Arena::from_raw(raw_parent_arena) });

      parent_arena.fuse(val.as_message_mut_inner(::protobuf::__internal::Private).arena());
      ::protobuf::__internal::runtime::upb_MessageValue { msg_val: Some(val.raw_msg()) }
    }

    unsafe fn from_message_value<'msg>(msg: ::protobuf::__internal::runtime::upb_MessageValue)
        -> ::protobuf::View<'msg, Self> {
        let raw = unsafe { msg.msg_val }.expect("expected present message value in map");
        let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
        PayloadEnemyDespawnView::new(::protobuf::__internal::Private, inner)
    }

    unsafe fn from_message_mut<'msg>(msg: ::protobuf::__internal::runtime::RawMessage, arena: &'msg ::protobuf::__internal::runtime::Arena)
        -> PayloadEnemyDespawnMut<'msg> {
        let inner = unsafe { ::protobuf::__internal::runtime::MessageMutInner::<'msg, PayloadEnemyDespawn>::wrap_raw(msg, arena) };
        PayloadEnemyDespawnMut::new(::protobuf::__internal::Private, inner)
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct PayloadEnemyDespawnMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, PayloadEnemyDespawn>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PayloadEnemyDespawnMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for PayloadEnemyDespawnMut<'msg> {
  type Message = PayloadEnemyDespawn;
}

impl ::std::fmt::Debug for PayloadEnemyDespawnMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    let string = unsafe {
      ::protobuf::__internal::runtime::debug_string(
        self.raw_msg(),
        <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table()
      )
    };
    write!(f, "{}", string)
  }
}

impl ::protobuf::Serialize for PayloadEnemyDespawnMut<'_> {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

#[allow(dead_code)]
impl<'msg> PayloadEnemyDespawnMut<'msg> {
  #[doc(hidden)]
  pub fn from_parent<ParentT: ::protobuf::Message>(
             _private: ::protobuf::__internal::Private,
             parent: ::protobuf::__internal::runtime::MessageMutInner<'msg, ParentT>,
             msg: ::protobuf::__internal::runtime::RawMessage)
    -> Self {
    Self {
      inner: ::protobuf::__internal::runtime::MessageMutInner::from_parent(parent, msg)
    }
  }

  #[doc(hidden)]
  pub fn new(_private: ::protobuf::__internal::Private, inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, PayloadEnemyDespawn>) -> Self {
    Self { inner }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, PayloadEnemyDespawn> {
    self.inner
  }

  pub fn to_owned(&self) -> PayloadEnemyDespawn {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  fn arena(&mut self) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }

  // id: optional uint64
  pub fn id(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        0, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_id(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        0, val.into()
      )
    }
  }

}

// SAFETY:
// - `PayloadEnemyDespawnMut` does not perform any shared mutation.
// - `PayloadEnemyDespawnMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for PayloadEnemyDespawnMut<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for PayloadEnemyDespawnMut<'msg> {}
impl<'msg> ::protobuf::MutProxy<'msg> for PayloadEnemyDespawnMut<'msg> {}

impl<'msg> ::protobuf::AsView for PayloadEnemyDespawnMut<'msg> {
  type Proxied = PayloadEnemyDespawn;
  fn as_view(&self) -> ::protobuf::View<'_, PayloadEnemyDespawn> {
    PayloadEnemyDespawnView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner.clone()),
      _phantom: ::std::marker::PhantomData
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PayloadEnemyDespawnMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, PayloadEnemyDespawn>
  where
      'msg: 'shorter {
    PayloadEnemyDespawnView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner.clone()),
      _phantom: ::std::marker::PhantomData
    }
  }
}

impl<'msg> ::protobuf::AsMut for PayloadEnemyDespawnMut<'msg> {
  type MutProxied = PayloadEnemyDespawn;
  fn as_mut(&mut self) -> PayloadEnemyDespawnMut<'msg> {
    PayloadEnemyDespawnMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for PayloadEnemyDespawnMut<'msg> {
  fn into_mut<'shorter>(self) -> PayloadEnemyDespawnMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl PayloadEnemyDespawn {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, PayloadEnemyDespawn> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  fn arena(&mut self) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }

  pub fn parse(data: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    let mut msg = Self::new();
    ::protobuf::ClearAndParse::clear_and_parse(&mut msg, data).map(|_| msg)
  }

  pub fn parse_dont_enforce_required(data: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    let mut msg = Self::new();
    ::protobuf::ClearAndParse::clear_and_parse_dont_enforce_required(&mut msg, data).map(|_| msg)
  }

  pub fn as_view(&self) -> PayloadEnemyDespawnView {
    PayloadEnemyDespawnView::new(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner))
  }

  pub fn as_mut(&mut self) -> PayloadEnemyDespawnMut {
    let inner = ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner);
    PayloadEnemyDespawnMut::new(::protobuf::__internal::Private, inner)
  }

  // id: optional uint64
  pub fn id(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        0, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_id(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        0, val.into()
      )
    }
  }

}  // impl PayloadEnemyDespawn

impl ::std::ops::Drop for PayloadEnemyDespawn {
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for PayloadEnemyDespawn {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for PayloadEnemyDespawn {
  type Proxied = Self;
  fn as_view(&self) -> PayloadEnemyDespawnView {
    self.as_view()
  }
}

impl ::protobuf::AsMut for PayloadEnemyDespawn {
  type MutProxied = Self;
  fn as_mut(&mut self) -> PayloadEnemyDespawnMut {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for PayloadEnemyDespawn {
  fn mini_table() -> *const ::protobuf::__internal::runtime::upb_MiniTable {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTablePtr> =
        ::std::sync::OnceLock::new();
    ONCE_LOCK.get_or_init(|| unsafe {
      super::Proto__PayloadEnemyDespawn_msg_init.0 =
          ::protobuf::__internal::runtime::upb_MiniTable_Build(
              "$,P".as_ptr(),
              3,
              ::protobuf::__internal::runtime::THREAD_LOCAL_ARENA.with(|a| a.raw()),
              ::std::ptr::null_mut());
      let submessages = [
      ];
      let subenums = [
      ];
      assert!(::protobuf::__internal::runtime::upb_MiniTable_Link(
          super::Proto__PayloadEnemyDespawn_msg_init.0,
          submessages.as_ptr() as *const *const ::protobuf::__internal::runtime::upb_MiniTable,
          submessages.len(), subenums.as_ptr(), subenums.len()));
      ::protobuf::__internal::runtime::MiniTablePtr(super::Proto__PayloadEnemyDespawn_msg_init.0)
    }).0
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for PayloadEnemyDespawn {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for PayloadEnemyDespawnView<'_> {
  #[inline(always)]
  fn mini_table() -> *const ::protobuf::__internal::runtime::upb_MiniTable {
    <PayloadEnemyDespawn as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for PayloadEnemyDespawnMut<'_> {
  #[inline(always)]
  fn mini_table() -> *const ::protobuf::__internal::runtime::upb_MiniTable {
    <PayloadEnemyDespawn as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for PayloadEnemyDespawn {
  type Msg = PayloadEnemyDespawn;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PayloadEnemyDespawn> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PayloadEnemyDespawn {
  type Msg = PayloadEnemyDespawn;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PayloadEnemyDespawn> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for PayloadEnemyDespawnMut<'_> {
  type Msg = PayloadEnemyDespawn;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PayloadEnemyDespawn> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PayloadEnemyDespawnMut<'_> {
  type Msg = PayloadEnemyDespawn;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PayloadEnemyDespawn> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PayloadEnemyDespawnView<'_> {
  type Msg = PayloadEnemyDespawn;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PayloadEnemyDespawn> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for PayloadEnemyDespawnMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}


// upb kernel doesn't support any owned message or message mut interop.
impl ::protobuf::OwnedMessageInterop for PayloadEnemyDespawn {}
impl<'a> ::protobuf::MessageMutInterop<'a> for PayloadEnemyDespawnMut<'a> {}

impl<'a> ::protobuf::MessageViewInterop<'a> for PayloadEnemyDespawnView<'a> {
  unsafe fn __unstable_wrap_raw_message(
    msg: &'a *const ::std::ffi::c_void) -> Self {
    let raw = ::protobuf::__internal::runtime::RawMessage::new(*msg as *mut _).unwrap();
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    Self::new(::protobuf::__internal::Private, inner)
  }
  unsafe fn __unstable_wrap_raw_message_unchecked_lifetime(
    msg: *const ::std::ffi::c_void) -> Self {
    let raw = ::protobuf::__internal::runtime::RawMessage::new(msg as *mut _).unwrap();
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    Self::new(::protobuf::__internal::Private, inner)
  }
  fn __unstable_as_raw_message(&self) -> *const ::std::ffi::c_void {
    self.inner.raw().as_ptr() as *const _
  }
}

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CategoryEnemyMessage(i32);

#[allow(non_upper_case_globals)]
impl CategoryEnemyMessage {
  pub const EnemySpawn: CategoryEnemyMessage = CategoryEnemyMessage(0);
  pub const EnemyDespawn: CategoryEnemyMessage = CategoryEnemyMessage(1);

  fn constant_name(&self) -> ::std::option::Option<&'static str> {
    #[allow(unreachable_patterns)] // In the case of aliases, just emit them all and let the first one match.
    Some(match self.0 {
      0 => "EnemySpawn",
      1 => "EnemyDespawn",
      _ => return None
    })
  }
}

impl ::std::convert::From<CategoryEnemyMessage> for i32 {
  fn from(val: CategoryEnemyMessage) -> i32 {
    val.0
  }
}

impl ::std::convert::From<i32> for CategoryEnemyMessage {
  fn from(val: i32) -> CategoryEnemyMessage {
    Self(val)
  }
}

impl ::std::default::Default for CategoryEnemyMessage {
  fn default() -> Self {
    Self(0)
  }
}

impl ::std::fmt::Debug for CategoryEnemyMessage {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    if let Some(constant_name) = self.constant_name() {
      write!(f, "CategoryEnemyMessage::{}", constant_name)
    } else {
      write!(f, "CategoryEnemyMessage::from({})", self.0)
    }
  }
}

impl ::protobuf::IntoProxied<i32> for CategoryEnemyMessage {
  fn into_proxied(self, _: ::protobuf::__internal::Private) -> i32 {
    self.0
  }
}

impl ::protobuf::__internal::SealedInternal for CategoryEnemyMessage {}

impl ::protobuf::Proxied for CategoryEnemyMessage {
  type View<'a> = CategoryEnemyMessage;
}

impl ::protobuf::Proxy<'_> for CategoryEnemyMessage {}
impl ::protobuf::ViewProxy<'_> for CategoryEnemyMessage {}

impl ::protobuf::AsView for CategoryEnemyMessage {
  type Proxied = CategoryEnemyMessage;

  fn as_view(&self) -> CategoryEnemyMessage {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for CategoryEnemyMessage {
  fn into_view<'shorter>(self) -> CategoryEnemyMessage where 'msg: 'shorter {
    self
  }
}

unsafe impl ::protobuf::ProxiedInRepeated for CategoryEnemyMessage {
  fn repeated_new(_private: ::protobuf::__internal::Private) -> ::protobuf::Repeated<Self> {
    ::protobuf::__internal::runtime::new_enum_repeated()
  }

  unsafe fn repeated_free(_private: ::protobuf::__internal::Private, f: &mut ::protobuf::Repeated<Self>) {
    ::protobuf::__internal::runtime::free_enum_repeated(f)
  }

  fn repeated_len(r: ::protobuf::View<::protobuf::Repeated<Self>>) -> usize {
    ::protobuf::__internal::runtime::cast_enum_repeated_view(r).len()
  }

  fn repeated_push(r: ::protobuf::Mut<::protobuf::Repeated<Self>>, val: impl ::protobuf::IntoProxied<CategoryEnemyMessage>) {
    ::protobuf::__internal::runtime::cast_enum_repeated_mut(r).push(val.into_proxied(::protobuf::__internal::Private))
  }

  fn repeated_clear(r: ::protobuf::Mut<::protobuf::Repeated<Self>>) {
    ::protobuf::__internal::runtime::cast_enum_repeated_mut(r).clear()
  }

  unsafe fn repeated_get_unchecked(
      r: ::protobuf::View<::protobuf::Repeated<Self>>,
      index: usize,
  ) -> ::protobuf::View<CategoryEnemyMessage> {
    // SAFETY: In-bounds as promised by the caller.
    unsafe {
      ::protobuf::__internal::runtime::cast_enum_repeated_view(r)
        .get_unchecked(index)
        .try_into()
        .unwrap_unchecked()
    }
  }

  unsafe fn repeated_set_unchecked(
      r: ::protobuf::Mut<::protobuf::Repeated<Self>>,
      index: usize,
      val: impl ::protobuf::IntoProxied<CategoryEnemyMessage>,
  ) {
    // SAFETY: In-bounds as promised by the caller.
    unsafe {
      ::protobuf::__internal::runtime::cast_enum_repeated_mut(r)
        .set_unchecked(index, val.into_proxied(::protobuf::__internal::Private))
    }
  }

  fn repeated_copy_from(
      src: ::protobuf::View<::protobuf::Repeated<Self>>,
      dest: ::protobuf::Mut<::protobuf::Repeated<Self>>,
  ) {
    ::protobuf::__internal::runtime::cast_enum_repeated_mut(dest)
      .copy_from(::protobuf::__internal::runtime::cast_enum_repeated_view(src))
  }

  fn repeated_reserve(
      r: ::protobuf::Mut<::protobuf::Repeated<Self>>,
      additional: usize,
  ) {
      // SAFETY:
      // - `f.as_raw()` is valid.
      ::protobuf::__internal::runtime::reserve_enum_repeated_mut(r, additional);
  }
}

// SAFETY: this is an enum type
unsafe impl ::protobuf::__internal::Enum for CategoryEnemyMessage {
  const NAME: &'static str = "CategoryEnemyMessage";

  fn is_known(value: i32) -> bool {
    matches!(value, 0|1)
  }
}

impl ::protobuf::__internal::runtime::UpbTypeConversions for CategoryEnemyMessage {
    fn upb_type() -> ::protobuf::__internal::runtime::CType {
        ::protobuf::__internal::runtime::CType::Enum
    }

    fn to_message_value(
        val: ::protobuf::View<'_, Self>) -> ::protobuf::__internal::runtime::upb_MessageValue {
        ::protobuf::__internal::runtime::upb_MessageValue { int32_val: val.0 }
    }

    unsafe fn into_message_value_fuse_if_required(
      _raw_parent_arena: ::protobuf::__internal::runtime::RawArena,
      val: Self) -> ::protobuf::__internal::runtime::upb_MessageValue {
        ::protobuf::__internal::runtime::upb_MessageValue { int32_val: val.0 }
    }

    unsafe fn from_message_value<'msg>(val: ::protobuf::__internal::runtime::upb_MessageValue)
        -> ::protobuf::View<'msg, Self> {
      CategoryEnemyMessage(unsafe { val.int32_val })
    }
}


