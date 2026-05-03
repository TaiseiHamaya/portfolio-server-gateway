const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.32.1-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut Proto__PayloadTransformSync_msg_init: ::protobuf::__internal::runtime::MiniTablePtr =
    ::protobuf::__internal::runtime::MiniTablePtr(::std::ptr::null_mut());
#[allow(non_camel_case_types)]
pub struct PayloadTransformSync {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<PayloadTransformSync>
}

impl ::protobuf::Message for PayloadTransformSync {}

impl ::std::default::Default for PayloadTransformSync {
  fn default() -> Self {
    Self::new()
  }
}

impl ::protobuf::Parse for PayloadTransformSync {
  fn parse(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    Self::parse(serialized)
  }

  fn parse_dont_enforce_required(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    Self::parse_dont_enforce_required(serialized)
  }
}

impl ::std::fmt::Debug for PayloadTransformSync {
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

impl ::protobuf::Serialize for PayloadTransformSync {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

// SAFETY:
// - `PayloadTransformSync` is `Sync` because it does not implement interior mutability.
//    Neither does `PayloadTransformSyncMut`.
unsafe impl Sync for PayloadTransformSync {}

// SAFETY:
// - `PayloadTransformSync` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for PayloadTransformSync {}

impl ::protobuf::Proxied for PayloadTransformSync {
  type View<'msg> = PayloadTransformSyncView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for PayloadTransformSync {}

impl ::protobuf::MutProxied for PayloadTransformSync {
  type Mut<'msg> = PayloadTransformSyncMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct PayloadTransformSyncView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, PayloadTransformSync>,
  _phantom: ::std::marker::PhantomData<&'msg ()>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PayloadTransformSyncView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for PayloadTransformSyncView<'msg> {
  type Message = PayloadTransformSync;
}

impl ::std::fmt::Debug for PayloadTransformSyncView<'_> {
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

impl ::protobuf::Serialize for PayloadTransformSyncView<'_> {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    // SAFETY: `MINI_TABLE` is the one associated with `self.raw_msg()`.
    let encoded = unsafe {
      ::protobuf::__internal::runtime::wire::encode(self.raw_msg(),
          <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table())
    };
    encoded.map_err(|_| ::protobuf::SerializeError)
  }
}

impl ::std::default::Default for PayloadTransformSyncView<'_> {
  fn default() -> PayloadTransformSyncView<'static> {
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(::protobuf::__internal::runtime::ScratchSpace::zeroed_block()) };
    PayloadTransformSyncView::new(::protobuf::__internal::Private, inner)
  }
}

#[allow(dead_code)]
impl<'msg> PayloadTransformSyncView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::protobuf::__internal::Private, inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, PayloadTransformSync>) -> Self {
    Self { inner, _phantom: ::std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  pub fn to_owned(&self) -> PayloadTransformSync {
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

  // timestamp: optional uint64
  pub fn timestamp(self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        1, (0u64).into()
      ).try_into().unwrap()
    }
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
// - `PayloadTransformSyncView` is `Sync` because it does not support mutation.
unsafe impl Sync for PayloadTransformSyncView<'_> {}

// SAFETY:
// - `PayloadTransformSyncView` is `Send` because while its alive a `PayloadTransformSyncMut` cannot.
// - `PayloadTransformSyncView` does not use thread-local data.
unsafe impl Send for PayloadTransformSyncView<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for PayloadTransformSyncView<'msg> {}
impl<'msg> ::protobuf::ViewProxy<'msg> for PayloadTransformSyncView<'msg> {}

impl<'msg> ::protobuf::AsView for PayloadTransformSyncView<'msg> {
  type Proxied = PayloadTransformSync;
  fn as_view(&self) -> ::protobuf::View<'msg, PayloadTransformSync> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PayloadTransformSyncView<'msg> {
  fn into_view<'shorter>(self) -> PayloadTransformSyncView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<PayloadTransformSync> for PayloadTransformSyncView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> PayloadTransformSync {
    let mut dst = PayloadTransformSync::new();
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

impl<'msg> ::protobuf::IntoProxied<PayloadTransformSync> for PayloadTransformSyncMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> PayloadTransformSync {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::UpbTypeConversions for PayloadTransformSync {
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
        PayloadTransformSyncView::new(::protobuf::__internal::Private, inner)
    }

    unsafe fn from_message_mut<'msg>(msg: ::protobuf::__internal::runtime::RawMessage, arena: &'msg ::protobuf::__internal::runtime::Arena)
        -> PayloadTransformSyncMut<'msg> {
        let inner = unsafe { ::protobuf::__internal::runtime::MessageMutInner::<'msg, PayloadTransformSync>::wrap_raw(msg, arena) };
        PayloadTransformSyncMut::new(::protobuf::__internal::Private, inner)
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct PayloadTransformSyncMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, PayloadTransformSync>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PayloadTransformSyncMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for PayloadTransformSyncMut<'msg> {
  type Message = PayloadTransformSync;
}

impl ::std::fmt::Debug for PayloadTransformSyncMut<'_> {
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

impl ::protobuf::Serialize for PayloadTransformSyncMut<'_> {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

#[allow(dead_code)]
impl<'msg> PayloadTransformSyncMut<'msg> {
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
  pub fn new(_private: ::protobuf::__internal::Private, inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, PayloadTransformSync>) -> Self {
    Self { inner }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, PayloadTransformSync> {
    self.inner
  }

  pub fn to_owned(&self) -> PayloadTransformSync {
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

  // timestamp: optional uint64
  pub fn timestamp(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        1, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_timestamp(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        1, val.into()
      )
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
// - `PayloadTransformSyncMut` does not perform any shared mutation.
// - `PayloadTransformSyncMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for PayloadTransformSyncMut<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for PayloadTransformSyncMut<'msg> {}
impl<'msg> ::protobuf::MutProxy<'msg> for PayloadTransformSyncMut<'msg> {}

impl<'msg> ::protobuf::AsView for PayloadTransformSyncMut<'msg> {
  type Proxied = PayloadTransformSync;
  fn as_view(&self) -> ::protobuf::View<'_, PayloadTransformSync> {
    PayloadTransformSyncView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner.clone()),
      _phantom: ::std::marker::PhantomData
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PayloadTransformSyncMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, PayloadTransformSync>
  where
      'msg: 'shorter {
    PayloadTransformSyncView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner.clone()),
      _phantom: ::std::marker::PhantomData
    }
  }
}

impl<'msg> ::protobuf::AsMut for PayloadTransformSyncMut<'msg> {
  type MutProxied = PayloadTransformSync;
  fn as_mut(&mut self) -> PayloadTransformSyncMut<'msg> {
    PayloadTransformSyncMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for PayloadTransformSyncMut<'msg> {
  fn into_mut<'shorter>(self) -> PayloadTransformSyncMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl PayloadTransformSync {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, PayloadTransformSync> {
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

  pub fn as_view(&self) -> PayloadTransformSyncView {
    PayloadTransformSyncView::new(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner))
  }

  pub fn as_mut(&mut self) -> PayloadTransformSyncMut {
    let inner = ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner);
    PayloadTransformSyncMut::new(::protobuf::__internal::Private, inner)
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

  // timestamp: optional uint64
  pub fn timestamp(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        1, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_timestamp(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        1, val.into()
      )
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

}  // impl PayloadTransformSync

impl ::std::ops::Drop for PayloadTransformSync {
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for PayloadTransformSync {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for PayloadTransformSync {
  type Proxied = Self;
  fn as_view(&self) -> PayloadTransformSyncView {
    self.as_view()
  }
}

impl ::protobuf::AsMut for PayloadTransformSync {
  type MutProxied = Self;
  fn as_mut(&mut self) -> PayloadTransformSyncMut {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for PayloadTransformSync {
  fn mini_table() -> *const ::protobuf::__internal::runtime::upb_MiniTable {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTablePtr> =
        ::std::sync::OnceLock::new();
    ONCE_LOCK.get_or_init(|| unsafe {
      super::Proto__PayloadTransformSync_msg_init.0 =
          ::protobuf::__internal::runtime::upb_MiniTable_Build(
              "$,P,P3".as_ptr(),
              6,
              ::protobuf::__internal::runtime::THREAD_LOCAL_ARENA.with(|a| a.raw()),
              ::std::ptr::null_mut());
      let submessages = [
        <super::Vector3 as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
      ];
      let subenums = [
      ];
      assert!(::protobuf::__internal::runtime::upb_MiniTable_Link(
          super::Proto__PayloadTransformSync_msg_init.0,
          submessages.as_ptr() as *const *const ::protobuf::__internal::runtime::upb_MiniTable,
          submessages.len(), subenums.as_ptr(), subenums.len()));
      ::protobuf::__internal::runtime::MiniTablePtr(super::Proto__PayloadTransformSync_msg_init.0)
    }).0
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for PayloadTransformSync {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for PayloadTransformSyncView<'_> {
  #[inline(always)]
  fn mini_table() -> *const ::protobuf::__internal::runtime::upb_MiniTable {
    <PayloadTransformSync as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for PayloadTransformSyncMut<'_> {
  #[inline(always)]
  fn mini_table() -> *const ::protobuf::__internal::runtime::upb_MiniTable {
    <PayloadTransformSync as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for PayloadTransformSync {
  type Msg = PayloadTransformSync;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PayloadTransformSync> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PayloadTransformSync {
  type Msg = PayloadTransformSync;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PayloadTransformSync> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for PayloadTransformSyncMut<'_> {
  type Msg = PayloadTransformSync;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PayloadTransformSync> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PayloadTransformSyncMut<'_> {
  type Msg = PayloadTransformSync;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PayloadTransformSync> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PayloadTransformSyncView<'_> {
  type Msg = PayloadTransformSync;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PayloadTransformSync> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for PayloadTransformSyncMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}


// upb kernel doesn't support any owned message or message mut interop.
impl ::protobuf::OwnedMessageInterop for PayloadTransformSync {}
impl<'a> ::protobuf::MessageMutInterop<'a> for PayloadTransformSyncMut<'a> {}

impl<'a> ::protobuf::MessageViewInterop<'a> for PayloadTransformSyncView<'a> {
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
pub(crate) static mut Proto__PayloadPlayAction_msg_init: ::protobuf::__internal::runtime::MiniTablePtr =
    ::protobuf::__internal::runtime::MiniTablePtr(::std::ptr::null_mut());
#[allow(non_camel_case_types)]
pub struct PayloadPlayAction {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<PayloadPlayAction>
}

impl ::protobuf::Message for PayloadPlayAction {}

impl ::std::default::Default for PayloadPlayAction {
  fn default() -> Self {
    Self::new()
  }
}

impl ::protobuf::Parse for PayloadPlayAction {
  fn parse(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    Self::parse(serialized)
  }

  fn parse_dont_enforce_required(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    Self::parse_dont_enforce_required(serialized)
  }
}

impl ::std::fmt::Debug for PayloadPlayAction {
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

impl ::protobuf::Serialize for PayloadPlayAction {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

// SAFETY:
// - `PayloadPlayAction` is `Sync` because it does not implement interior mutability.
//    Neither does `PayloadPlayActionMut`.
unsafe impl Sync for PayloadPlayAction {}

// SAFETY:
// - `PayloadPlayAction` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for PayloadPlayAction {}

impl ::protobuf::Proxied for PayloadPlayAction {
  type View<'msg> = PayloadPlayActionView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for PayloadPlayAction {}

impl ::protobuf::MutProxied for PayloadPlayAction {
  type Mut<'msg> = PayloadPlayActionMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct PayloadPlayActionView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, PayloadPlayAction>,
  _phantom: ::std::marker::PhantomData<&'msg ()>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PayloadPlayActionView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for PayloadPlayActionView<'msg> {
  type Message = PayloadPlayAction;
}

impl ::std::fmt::Debug for PayloadPlayActionView<'_> {
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

impl ::protobuf::Serialize for PayloadPlayActionView<'_> {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    // SAFETY: `MINI_TABLE` is the one associated with `self.raw_msg()`.
    let encoded = unsafe {
      ::protobuf::__internal::runtime::wire::encode(self.raw_msg(),
          <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table())
    };
    encoded.map_err(|_| ::protobuf::SerializeError)
  }
}

impl ::std::default::Default for PayloadPlayActionView<'_> {
  fn default() -> PayloadPlayActionView<'static> {
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(::protobuf::__internal::runtime::ScratchSpace::zeroed_block()) };
    PayloadPlayActionView::new(::protobuf::__internal::Private, inner)
  }
}

#[allow(dead_code)]
impl<'msg> PayloadPlayActionView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::protobuf::__internal::Private, inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, PayloadPlayAction>) -> Self {
    Self { inner, _phantom: ::std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  pub fn to_owned(&self) -> PayloadPlayAction {
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

  // target_id: optional uint64
  pub fn target_id(self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        1, (0u64).into()
      ).try_into().unwrap()
    }
  }

  // action_id: optional uint32
  pub fn action_id(self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        2, (0u32).into()
      ).try_into().unwrap()
    }
  }

  // timestamp: optional int64
  pub fn timestamp(self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        3, (0i64).into()
      ).try_into().unwrap()
    }
  }

}

// SAFETY:
// - `PayloadPlayActionView` is `Sync` because it does not support mutation.
unsafe impl Sync for PayloadPlayActionView<'_> {}

// SAFETY:
// - `PayloadPlayActionView` is `Send` because while its alive a `PayloadPlayActionMut` cannot.
// - `PayloadPlayActionView` does not use thread-local data.
unsafe impl Send for PayloadPlayActionView<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for PayloadPlayActionView<'msg> {}
impl<'msg> ::protobuf::ViewProxy<'msg> for PayloadPlayActionView<'msg> {}

impl<'msg> ::protobuf::AsView for PayloadPlayActionView<'msg> {
  type Proxied = PayloadPlayAction;
  fn as_view(&self) -> ::protobuf::View<'msg, PayloadPlayAction> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PayloadPlayActionView<'msg> {
  fn into_view<'shorter>(self) -> PayloadPlayActionView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<PayloadPlayAction> for PayloadPlayActionView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> PayloadPlayAction {
    let mut dst = PayloadPlayAction::new();
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

impl<'msg> ::protobuf::IntoProxied<PayloadPlayAction> for PayloadPlayActionMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> PayloadPlayAction {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::UpbTypeConversions for PayloadPlayAction {
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
        PayloadPlayActionView::new(::protobuf::__internal::Private, inner)
    }

    unsafe fn from_message_mut<'msg>(msg: ::protobuf::__internal::runtime::RawMessage, arena: &'msg ::protobuf::__internal::runtime::Arena)
        -> PayloadPlayActionMut<'msg> {
        let inner = unsafe { ::protobuf::__internal::runtime::MessageMutInner::<'msg, PayloadPlayAction>::wrap_raw(msg, arena) };
        PayloadPlayActionMut::new(::protobuf::__internal::Private, inner)
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct PayloadPlayActionMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, PayloadPlayAction>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PayloadPlayActionMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for PayloadPlayActionMut<'msg> {
  type Message = PayloadPlayAction;
}

impl ::std::fmt::Debug for PayloadPlayActionMut<'_> {
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

impl ::protobuf::Serialize for PayloadPlayActionMut<'_> {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

#[allow(dead_code)]
impl<'msg> PayloadPlayActionMut<'msg> {
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
  pub fn new(_private: ::protobuf::__internal::Private, inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, PayloadPlayAction>) -> Self {
    Self { inner }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, PayloadPlayAction> {
    self.inner
  }

  pub fn to_owned(&self) -> PayloadPlayAction {
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

  // target_id: optional uint64
  pub fn target_id(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        1, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_target_id(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        1, val.into()
      )
    }
  }

  // action_id: optional uint32
  pub fn action_id(&self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        2, (0u32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_action_id(&mut self, val: u32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u32_at_index(
        2, val.into()
      )
    }
  }

  // timestamp: optional int64
  pub fn timestamp(&self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        3, (0i64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_timestamp(&mut self, val: i64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i64_at_index(
        3, val.into()
      )
    }
  }

}

// SAFETY:
// - `PayloadPlayActionMut` does not perform any shared mutation.
// - `PayloadPlayActionMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for PayloadPlayActionMut<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for PayloadPlayActionMut<'msg> {}
impl<'msg> ::protobuf::MutProxy<'msg> for PayloadPlayActionMut<'msg> {}

impl<'msg> ::protobuf::AsView for PayloadPlayActionMut<'msg> {
  type Proxied = PayloadPlayAction;
  fn as_view(&self) -> ::protobuf::View<'_, PayloadPlayAction> {
    PayloadPlayActionView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner.clone()),
      _phantom: ::std::marker::PhantomData
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PayloadPlayActionMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, PayloadPlayAction>
  where
      'msg: 'shorter {
    PayloadPlayActionView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner.clone()),
      _phantom: ::std::marker::PhantomData
    }
  }
}

impl<'msg> ::protobuf::AsMut for PayloadPlayActionMut<'msg> {
  type MutProxied = PayloadPlayAction;
  fn as_mut(&mut self) -> PayloadPlayActionMut<'msg> {
    PayloadPlayActionMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for PayloadPlayActionMut<'msg> {
  fn into_mut<'shorter>(self) -> PayloadPlayActionMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl PayloadPlayAction {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, PayloadPlayAction> {
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

  pub fn as_view(&self) -> PayloadPlayActionView {
    PayloadPlayActionView::new(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner))
  }

  pub fn as_mut(&mut self) -> PayloadPlayActionMut {
    let inner = ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner);
    PayloadPlayActionMut::new(::protobuf::__internal::Private, inner)
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

  // target_id: optional uint64
  pub fn target_id(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        1, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_target_id(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        1, val.into()
      )
    }
  }

  // action_id: optional uint32
  pub fn action_id(&self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        2, (0u32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_action_id(&mut self, val: u32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u32_at_index(
        2, val.into()
      )
    }
  }

  // timestamp: optional int64
  pub fn timestamp(&self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        3, (0i64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_timestamp(&mut self, val: i64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i64_at_index(
        3, val.into()
      )
    }
  }

}  // impl PayloadPlayAction

impl ::std::ops::Drop for PayloadPlayAction {
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for PayloadPlayAction {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for PayloadPlayAction {
  type Proxied = Self;
  fn as_view(&self) -> PayloadPlayActionView {
    self.as_view()
  }
}

impl ::protobuf::AsMut for PayloadPlayAction {
  type MutProxied = Self;
  fn as_mut(&mut self) -> PayloadPlayActionMut {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for PayloadPlayAction {
  fn mini_table() -> *const ::protobuf::__internal::runtime::upb_MiniTable {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTablePtr> =
        ::std::sync::OnceLock::new();
    ONCE_LOCK.get_or_init(|| unsafe {
      super::Proto__PayloadPlayAction_msg_init.0 =
          ::protobuf::__internal::runtime::upb_MiniTable_Build(
              "$,P,P)P+P".as_ptr(),
              9,
              ::protobuf::__internal::runtime::THREAD_LOCAL_ARENA.with(|a| a.raw()),
              ::std::ptr::null_mut());
      let submessages = [
      ];
      let subenums = [
      ];
      assert!(::protobuf::__internal::runtime::upb_MiniTable_Link(
          super::Proto__PayloadPlayAction_msg_init.0,
          submessages.as_ptr() as *const *const ::protobuf::__internal::runtime::upb_MiniTable,
          submessages.len(), subenums.as_ptr(), subenums.len()));
      ::protobuf::__internal::runtime::MiniTablePtr(super::Proto__PayloadPlayAction_msg_init.0)
    }).0
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for PayloadPlayAction {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for PayloadPlayActionView<'_> {
  #[inline(always)]
  fn mini_table() -> *const ::protobuf::__internal::runtime::upb_MiniTable {
    <PayloadPlayAction as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for PayloadPlayActionMut<'_> {
  #[inline(always)]
  fn mini_table() -> *const ::protobuf::__internal::runtime::upb_MiniTable {
    <PayloadPlayAction as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for PayloadPlayAction {
  type Msg = PayloadPlayAction;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PayloadPlayAction> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PayloadPlayAction {
  type Msg = PayloadPlayAction;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PayloadPlayAction> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for PayloadPlayActionMut<'_> {
  type Msg = PayloadPlayAction;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PayloadPlayAction> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PayloadPlayActionMut<'_> {
  type Msg = PayloadPlayAction;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PayloadPlayAction> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PayloadPlayActionView<'_> {
  type Msg = PayloadPlayAction;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PayloadPlayAction> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for PayloadPlayActionMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}


// upb kernel doesn't support any owned message or message mut interop.
impl ::protobuf::OwnedMessageInterop for PayloadPlayAction {}
impl<'a> ::protobuf::MessageMutInterop<'a> for PayloadPlayActionMut<'a> {}

impl<'a> ::protobuf::MessageViewInterop<'a> for PayloadPlayActionView<'a> {
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
pub(crate) static mut Proto__PayloadEntityDamaged_msg_init: ::protobuf::__internal::runtime::MiniTablePtr =
    ::protobuf::__internal::runtime::MiniTablePtr(::std::ptr::null_mut());
#[allow(non_camel_case_types)]
pub struct PayloadEntityDamaged {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<PayloadEntityDamaged>
}

impl ::protobuf::Message for PayloadEntityDamaged {}

impl ::std::default::Default for PayloadEntityDamaged {
  fn default() -> Self {
    Self::new()
  }
}

impl ::protobuf::Parse for PayloadEntityDamaged {
  fn parse(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    Self::parse(serialized)
  }

  fn parse_dont_enforce_required(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    Self::parse_dont_enforce_required(serialized)
  }
}

impl ::std::fmt::Debug for PayloadEntityDamaged {
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

impl ::protobuf::Serialize for PayloadEntityDamaged {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

// SAFETY:
// - `PayloadEntityDamaged` is `Sync` because it does not implement interior mutability.
//    Neither does `PayloadEntityDamagedMut`.
unsafe impl Sync for PayloadEntityDamaged {}

// SAFETY:
// - `PayloadEntityDamaged` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for PayloadEntityDamaged {}

impl ::protobuf::Proxied for PayloadEntityDamaged {
  type View<'msg> = PayloadEntityDamagedView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for PayloadEntityDamaged {}

impl ::protobuf::MutProxied for PayloadEntityDamaged {
  type Mut<'msg> = PayloadEntityDamagedMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct PayloadEntityDamagedView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, PayloadEntityDamaged>,
  _phantom: ::std::marker::PhantomData<&'msg ()>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PayloadEntityDamagedView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for PayloadEntityDamagedView<'msg> {
  type Message = PayloadEntityDamaged;
}

impl ::std::fmt::Debug for PayloadEntityDamagedView<'_> {
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

impl ::protobuf::Serialize for PayloadEntityDamagedView<'_> {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    // SAFETY: `MINI_TABLE` is the one associated with `self.raw_msg()`.
    let encoded = unsafe {
      ::protobuf::__internal::runtime::wire::encode(self.raw_msg(),
          <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table())
    };
    encoded.map_err(|_| ::protobuf::SerializeError)
  }
}

impl ::std::default::Default for PayloadEntityDamagedView<'_> {
  fn default() -> PayloadEntityDamagedView<'static> {
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(::protobuf::__internal::runtime::ScratchSpace::zeroed_block()) };
    PayloadEntityDamagedView::new(::protobuf::__internal::Private, inner)
  }
}

#[allow(dead_code)]
impl<'msg> PayloadEntityDamagedView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::protobuf::__internal::Private, inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, PayloadEntityDamaged>) -> Self {
    Self { inner, _phantom: ::std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  pub fn to_owned(&self) -> PayloadEntityDamaged {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // entity_id: optional uint64
  pub fn entity_id(self) -> u64 {
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

  // damage: optional int32
  pub fn damage(self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        1, (0i32).into()
      ).try_into().unwrap()
    }
  }

  // current_hp: optional int32
  pub fn current_hp(self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        2, (0i32).into()
      ).try_into().unwrap()
    }
  }

}

// SAFETY:
// - `PayloadEntityDamagedView` is `Sync` because it does not support mutation.
unsafe impl Sync for PayloadEntityDamagedView<'_> {}

// SAFETY:
// - `PayloadEntityDamagedView` is `Send` because while its alive a `PayloadEntityDamagedMut` cannot.
// - `PayloadEntityDamagedView` does not use thread-local data.
unsafe impl Send for PayloadEntityDamagedView<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for PayloadEntityDamagedView<'msg> {}
impl<'msg> ::protobuf::ViewProxy<'msg> for PayloadEntityDamagedView<'msg> {}

impl<'msg> ::protobuf::AsView for PayloadEntityDamagedView<'msg> {
  type Proxied = PayloadEntityDamaged;
  fn as_view(&self) -> ::protobuf::View<'msg, PayloadEntityDamaged> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PayloadEntityDamagedView<'msg> {
  fn into_view<'shorter>(self) -> PayloadEntityDamagedView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<PayloadEntityDamaged> for PayloadEntityDamagedView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> PayloadEntityDamaged {
    let mut dst = PayloadEntityDamaged::new();
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

impl<'msg> ::protobuf::IntoProxied<PayloadEntityDamaged> for PayloadEntityDamagedMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> PayloadEntityDamaged {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::UpbTypeConversions for PayloadEntityDamaged {
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
        PayloadEntityDamagedView::new(::protobuf::__internal::Private, inner)
    }

    unsafe fn from_message_mut<'msg>(msg: ::protobuf::__internal::runtime::RawMessage, arena: &'msg ::protobuf::__internal::runtime::Arena)
        -> PayloadEntityDamagedMut<'msg> {
        let inner = unsafe { ::protobuf::__internal::runtime::MessageMutInner::<'msg, PayloadEntityDamaged>::wrap_raw(msg, arena) };
        PayloadEntityDamagedMut::new(::protobuf::__internal::Private, inner)
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct PayloadEntityDamagedMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, PayloadEntityDamaged>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PayloadEntityDamagedMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for PayloadEntityDamagedMut<'msg> {
  type Message = PayloadEntityDamaged;
}

impl ::std::fmt::Debug for PayloadEntityDamagedMut<'_> {
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

impl ::protobuf::Serialize for PayloadEntityDamagedMut<'_> {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

#[allow(dead_code)]
impl<'msg> PayloadEntityDamagedMut<'msg> {
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
  pub fn new(_private: ::protobuf::__internal::Private, inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, PayloadEntityDamaged>) -> Self {
    Self { inner }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, PayloadEntityDamaged> {
    self.inner
  }

  pub fn to_owned(&self) -> PayloadEntityDamaged {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  fn arena(&mut self) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }

  // entity_id: optional uint64
  pub fn entity_id(&self) -> u64 {
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
  pub fn set_entity_id(&mut self, val: u64) {
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

  // damage: optional int32
  pub fn damage(&self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        1, (0i32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_damage(&mut self, val: i32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        1, val.into()
      )
    }
  }

  // current_hp: optional int32
  pub fn current_hp(&self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        2, (0i32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_current_hp(&mut self, val: i32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        2, val.into()
      )
    }
  }

}

// SAFETY:
// - `PayloadEntityDamagedMut` does not perform any shared mutation.
// - `PayloadEntityDamagedMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for PayloadEntityDamagedMut<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for PayloadEntityDamagedMut<'msg> {}
impl<'msg> ::protobuf::MutProxy<'msg> for PayloadEntityDamagedMut<'msg> {}

impl<'msg> ::protobuf::AsView for PayloadEntityDamagedMut<'msg> {
  type Proxied = PayloadEntityDamaged;
  fn as_view(&self) -> ::protobuf::View<'_, PayloadEntityDamaged> {
    PayloadEntityDamagedView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner.clone()),
      _phantom: ::std::marker::PhantomData
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PayloadEntityDamagedMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, PayloadEntityDamaged>
  where
      'msg: 'shorter {
    PayloadEntityDamagedView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner.clone()),
      _phantom: ::std::marker::PhantomData
    }
  }
}

impl<'msg> ::protobuf::AsMut for PayloadEntityDamagedMut<'msg> {
  type MutProxied = PayloadEntityDamaged;
  fn as_mut(&mut self) -> PayloadEntityDamagedMut<'msg> {
    PayloadEntityDamagedMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for PayloadEntityDamagedMut<'msg> {
  fn into_mut<'shorter>(self) -> PayloadEntityDamagedMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl PayloadEntityDamaged {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, PayloadEntityDamaged> {
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

  pub fn as_view(&self) -> PayloadEntityDamagedView {
    PayloadEntityDamagedView::new(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner))
  }

  pub fn as_mut(&mut self) -> PayloadEntityDamagedMut {
    let inner = ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner);
    PayloadEntityDamagedMut::new(::protobuf::__internal::Private, inner)
  }

  // entity_id: optional uint64
  pub fn entity_id(&self) -> u64 {
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
  pub fn set_entity_id(&mut self, val: u64) {
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

  // damage: optional int32
  pub fn damage(&self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        1, (0i32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_damage(&mut self, val: i32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        1, val.into()
      )
    }
  }

  // current_hp: optional int32
  pub fn current_hp(&self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        2, (0i32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_current_hp(&mut self, val: i32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        2, val.into()
      )
    }
  }

}  // impl PayloadEntityDamaged

impl ::std::ops::Drop for PayloadEntityDamaged {
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for PayloadEntityDamaged {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for PayloadEntityDamaged {
  type Proxied = Self;
  fn as_view(&self) -> PayloadEntityDamagedView {
    self.as_view()
  }
}

impl ::protobuf::AsMut for PayloadEntityDamaged {
  type MutProxied = Self;
  fn as_mut(&mut self) -> PayloadEntityDamagedMut {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for PayloadEntityDamaged {
  fn mini_table() -> *const ::protobuf::__internal::runtime::upb_MiniTable {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTablePtr> =
        ::std::sync::OnceLock::new();
    ONCE_LOCK.get_or_init(|| unsafe {
      super::Proto__PayloadEntityDamaged_msg_init.0 =
          ::protobuf::__internal::runtime::upb_MiniTable_Build(
              "$,P(P(P".as_ptr(),
              7,
              ::protobuf::__internal::runtime::THREAD_LOCAL_ARENA.with(|a| a.raw()),
              ::std::ptr::null_mut());
      let submessages = [
      ];
      let subenums = [
      ];
      assert!(::protobuf::__internal::runtime::upb_MiniTable_Link(
          super::Proto__PayloadEntityDamaged_msg_init.0,
          submessages.as_ptr() as *const *const ::protobuf::__internal::runtime::upb_MiniTable,
          submessages.len(), subenums.as_ptr(), subenums.len()));
      ::protobuf::__internal::runtime::MiniTablePtr(super::Proto__PayloadEntityDamaged_msg_init.0)
    }).0
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for PayloadEntityDamaged {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for PayloadEntityDamagedView<'_> {
  #[inline(always)]
  fn mini_table() -> *const ::protobuf::__internal::runtime::upb_MiniTable {
    <PayloadEntityDamaged as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for PayloadEntityDamagedMut<'_> {
  #[inline(always)]
  fn mini_table() -> *const ::protobuf::__internal::runtime::upb_MiniTable {
    <PayloadEntityDamaged as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for PayloadEntityDamaged {
  type Msg = PayloadEntityDamaged;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PayloadEntityDamaged> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PayloadEntityDamaged {
  type Msg = PayloadEntityDamaged;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PayloadEntityDamaged> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for PayloadEntityDamagedMut<'_> {
  type Msg = PayloadEntityDamaged;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PayloadEntityDamaged> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PayloadEntityDamagedMut<'_> {
  type Msg = PayloadEntityDamaged;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PayloadEntityDamaged> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PayloadEntityDamagedView<'_> {
  type Msg = PayloadEntityDamaged;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PayloadEntityDamaged> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for PayloadEntityDamagedMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}


// upb kernel doesn't support any owned message or message mut interop.
impl ::protobuf::OwnedMessageInterop for PayloadEntityDamaged {}
impl<'a> ::protobuf::MessageMutInterop<'a> for PayloadEntityDamagedMut<'a> {}

impl<'a> ::protobuf::MessageViewInterop<'a> for PayloadEntityDamagedView<'a> {
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
pub(crate) static mut Proto__PayloadZoneEnterNotification_msg_init: ::protobuf::__internal::runtime::MiniTablePtr =
    ::protobuf::__internal::runtime::MiniTablePtr(::std::ptr::null_mut());
#[allow(non_camel_case_types)]
pub struct PayloadZoneEnterNotification {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<PayloadZoneEnterNotification>
}

impl ::protobuf::Message for PayloadZoneEnterNotification {}

impl ::std::default::Default for PayloadZoneEnterNotification {
  fn default() -> Self {
    Self::new()
  }
}

impl ::protobuf::Parse for PayloadZoneEnterNotification {
  fn parse(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    Self::parse(serialized)
  }

  fn parse_dont_enforce_required(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    Self::parse_dont_enforce_required(serialized)
  }
}

impl ::std::fmt::Debug for PayloadZoneEnterNotification {
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

impl ::protobuf::Serialize for PayloadZoneEnterNotification {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

// SAFETY:
// - `PayloadZoneEnterNotification` is `Sync` because it does not implement interior mutability.
//    Neither does `PayloadZoneEnterNotificationMut`.
unsafe impl Sync for PayloadZoneEnterNotification {}

// SAFETY:
// - `PayloadZoneEnterNotification` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for PayloadZoneEnterNotification {}

impl ::protobuf::Proxied for PayloadZoneEnterNotification {
  type View<'msg> = PayloadZoneEnterNotificationView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for PayloadZoneEnterNotification {}

impl ::protobuf::MutProxied for PayloadZoneEnterNotification {
  type Mut<'msg> = PayloadZoneEnterNotificationMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct PayloadZoneEnterNotificationView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, PayloadZoneEnterNotification>,
  _phantom: ::std::marker::PhantomData<&'msg ()>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PayloadZoneEnterNotificationView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for PayloadZoneEnterNotificationView<'msg> {
  type Message = PayloadZoneEnterNotification;
}

impl ::std::fmt::Debug for PayloadZoneEnterNotificationView<'_> {
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

impl ::protobuf::Serialize for PayloadZoneEnterNotificationView<'_> {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    // SAFETY: `MINI_TABLE` is the one associated with `self.raw_msg()`.
    let encoded = unsafe {
      ::protobuf::__internal::runtime::wire::encode(self.raw_msg(),
          <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table())
    };
    encoded.map_err(|_| ::protobuf::SerializeError)
  }
}

impl ::std::default::Default for PayloadZoneEnterNotificationView<'_> {
  fn default() -> PayloadZoneEnterNotificationView<'static> {
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(::protobuf::__internal::runtime::ScratchSpace::zeroed_block()) };
    PayloadZoneEnterNotificationView::new(::protobuf::__internal::Private, inner)
  }
}

#[allow(dead_code)]
impl<'msg> PayloadZoneEnterNotificationView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::protobuf::__internal::Private, inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, PayloadZoneEnterNotification>) -> Self {
    Self { inner, _phantom: ::std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  pub fn to_owned(&self) -> PayloadZoneEnterNotification {
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

  // username: optional string
  pub fn username(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
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
// - `PayloadZoneEnterNotificationView` is `Sync` because it does not support mutation.
unsafe impl Sync for PayloadZoneEnterNotificationView<'_> {}

// SAFETY:
// - `PayloadZoneEnterNotificationView` is `Send` because while its alive a `PayloadZoneEnterNotificationMut` cannot.
// - `PayloadZoneEnterNotificationView` does not use thread-local data.
unsafe impl Send for PayloadZoneEnterNotificationView<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for PayloadZoneEnterNotificationView<'msg> {}
impl<'msg> ::protobuf::ViewProxy<'msg> for PayloadZoneEnterNotificationView<'msg> {}

impl<'msg> ::protobuf::AsView for PayloadZoneEnterNotificationView<'msg> {
  type Proxied = PayloadZoneEnterNotification;
  fn as_view(&self) -> ::protobuf::View<'msg, PayloadZoneEnterNotification> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PayloadZoneEnterNotificationView<'msg> {
  fn into_view<'shorter>(self) -> PayloadZoneEnterNotificationView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<PayloadZoneEnterNotification> for PayloadZoneEnterNotificationView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> PayloadZoneEnterNotification {
    let mut dst = PayloadZoneEnterNotification::new();
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

impl<'msg> ::protobuf::IntoProxied<PayloadZoneEnterNotification> for PayloadZoneEnterNotificationMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> PayloadZoneEnterNotification {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::UpbTypeConversions for PayloadZoneEnterNotification {
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
        PayloadZoneEnterNotificationView::new(::protobuf::__internal::Private, inner)
    }

    unsafe fn from_message_mut<'msg>(msg: ::protobuf::__internal::runtime::RawMessage, arena: &'msg ::protobuf::__internal::runtime::Arena)
        -> PayloadZoneEnterNotificationMut<'msg> {
        let inner = unsafe { ::protobuf::__internal::runtime::MessageMutInner::<'msg, PayloadZoneEnterNotification>::wrap_raw(msg, arena) };
        PayloadZoneEnterNotificationMut::new(::protobuf::__internal::Private, inner)
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct PayloadZoneEnterNotificationMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, PayloadZoneEnterNotification>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PayloadZoneEnterNotificationMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for PayloadZoneEnterNotificationMut<'msg> {
  type Message = PayloadZoneEnterNotification;
}

impl ::std::fmt::Debug for PayloadZoneEnterNotificationMut<'_> {
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

impl ::protobuf::Serialize for PayloadZoneEnterNotificationMut<'_> {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

#[allow(dead_code)]
impl<'msg> PayloadZoneEnterNotificationMut<'msg> {
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
  pub fn new(_private: ::protobuf::__internal::Private, inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, PayloadZoneEnterNotification>) -> Self {
    Self { inner }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, PayloadZoneEnterNotification> {
    self.inner
  }

  pub fn to_owned(&self) -> PayloadZoneEnterNotification {
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

  // username: optional string
  pub fn username(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_username(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
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
// - `PayloadZoneEnterNotificationMut` does not perform any shared mutation.
// - `PayloadZoneEnterNotificationMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for PayloadZoneEnterNotificationMut<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for PayloadZoneEnterNotificationMut<'msg> {}
impl<'msg> ::protobuf::MutProxy<'msg> for PayloadZoneEnterNotificationMut<'msg> {}

impl<'msg> ::protobuf::AsView for PayloadZoneEnterNotificationMut<'msg> {
  type Proxied = PayloadZoneEnterNotification;
  fn as_view(&self) -> ::protobuf::View<'_, PayloadZoneEnterNotification> {
    PayloadZoneEnterNotificationView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner.clone()),
      _phantom: ::std::marker::PhantomData
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PayloadZoneEnterNotificationMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, PayloadZoneEnterNotification>
  where
      'msg: 'shorter {
    PayloadZoneEnterNotificationView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner.clone()),
      _phantom: ::std::marker::PhantomData
    }
  }
}

impl<'msg> ::protobuf::AsMut for PayloadZoneEnterNotificationMut<'msg> {
  type MutProxied = PayloadZoneEnterNotification;
  fn as_mut(&mut self) -> PayloadZoneEnterNotificationMut<'msg> {
    PayloadZoneEnterNotificationMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for PayloadZoneEnterNotificationMut<'msg> {
  fn into_mut<'shorter>(self) -> PayloadZoneEnterNotificationMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl PayloadZoneEnterNotification {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, PayloadZoneEnterNotification> {
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

  pub fn as_view(&self) -> PayloadZoneEnterNotificationView {
    PayloadZoneEnterNotificationView::new(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner))
  }

  pub fn as_mut(&mut self) -> PayloadZoneEnterNotificationMut {
    let inner = ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner);
    PayloadZoneEnterNotificationMut::new(::protobuf::__internal::Private, inner)
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

  // username: optional string
  pub fn username(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_username(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
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

}  // impl PayloadZoneEnterNotification

impl ::std::ops::Drop for PayloadZoneEnterNotification {
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for PayloadZoneEnterNotification {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for PayloadZoneEnterNotification {
  type Proxied = Self;
  fn as_view(&self) -> PayloadZoneEnterNotificationView {
    self.as_view()
  }
}

impl ::protobuf::AsMut for PayloadZoneEnterNotification {
  type MutProxied = Self;
  fn as_mut(&mut self) -> PayloadZoneEnterNotificationMut {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for PayloadZoneEnterNotification {
  fn mini_table() -> *const ::protobuf::__internal::runtime::upb_MiniTable {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTablePtr> =
        ::std::sync::OnceLock::new();
    ONCE_LOCK.get_or_init(|| unsafe {
      super::Proto__PayloadZoneEnterNotification_msg_init.0 =
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
          super::Proto__PayloadZoneEnterNotification_msg_init.0,
          submessages.as_ptr() as *const *const ::protobuf::__internal::runtime::upb_MiniTable,
          submessages.len(), subenums.as_ptr(), subenums.len()));
      ::protobuf::__internal::runtime::MiniTablePtr(super::Proto__PayloadZoneEnterNotification_msg_init.0)
    }).0
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for PayloadZoneEnterNotification {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for PayloadZoneEnterNotificationView<'_> {
  #[inline(always)]
  fn mini_table() -> *const ::protobuf::__internal::runtime::upb_MiniTable {
    <PayloadZoneEnterNotification as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for PayloadZoneEnterNotificationMut<'_> {
  #[inline(always)]
  fn mini_table() -> *const ::protobuf::__internal::runtime::upb_MiniTable {
    <PayloadZoneEnterNotification as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for PayloadZoneEnterNotification {
  type Msg = PayloadZoneEnterNotification;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PayloadZoneEnterNotification> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PayloadZoneEnterNotification {
  type Msg = PayloadZoneEnterNotification;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PayloadZoneEnterNotification> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for PayloadZoneEnterNotificationMut<'_> {
  type Msg = PayloadZoneEnterNotification;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PayloadZoneEnterNotification> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PayloadZoneEnterNotificationMut<'_> {
  type Msg = PayloadZoneEnterNotification;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PayloadZoneEnterNotification> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PayloadZoneEnterNotificationView<'_> {
  type Msg = PayloadZoneEnterNotification;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PayloadZoneEnterNotification> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for PayloadZoneEnterNotificationMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}


// upb kernel doesn't support any owned message or message mut interop.
impl ::protobuf::OwnedMessageInterop for PayloadZoneEnterNotification {}
impl<'a> ::protobuf::MessageMutInterop<'a> for PayloadZoneEnterNotificationMut<'a> {}

impl<'a> ::protobuf::MessageViewInterop<'a> for PayloadZoneEnterNotificationView<'a> {
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
pub(crate) static mut Proto__PayloadZoneExitNotification_msg_init: ::protobuf::__internal::runtime::MiniTablePtr =
    ::protobuf::__internal::runtime::MiniTablePtr(::std::ptr::null_mut());
#[allow(non_camel_case_types)]
pub struct PayloadZoneExitNotification {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<PayloadZoneExitNotification>
}

impl ::protobuf::Message for PayloadZoneExitNotification {}

impl ::std::default::Default for PayloadZoneExitNotification {
  fn default() -> Self {
    Self::new()
  }
}

impl ::protobuf::Parse for PayloadZoneExitNotification {
  fn parse(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    Self::parse(serialized)
  }

  fn parse_dont_enforce_required(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    Self::parse_dont_enforce_required(serialized)
  }
}

impl ::std::fmt::Debug for PayloadZoneExitNotification {
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

impl ::protobuf::Serialize for PayloadZoneExitNotification {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

// SAFETY:
// - `PayloadZoneExitNotification` is `Sync` because it does not implement interior mutability.
//    Neither does `PayloadZoneExitNotificationMut`.
unsafe impl Sync for PayloadZoneExitNotification {}

// SAFETY:
// - `PayloadZoneExitNotification` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for PayloadZoneExitNotification {}

impl ::protobuf::Proxied for PayloadZoneExitNotification {
  type View<'msg> = PayloadZoneExitNotificationView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for PayloadZoneExitNotification {}

impl ::protobuf::MutProxied for PayloadZoneExitNotification {
  type Mut<'msg> = PayloadZoneExitNotificationMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct PayloadZoneExitNotificationView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, PayloadZoneExitNotification>,
  _phantom: ::std::marker::PhantomData<&'msg ()>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PayloadZoneExitNotificationView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for PayloadZoneExitNotificationView<'msg> {
  type Message = PayloadZoneExitNotification;
}

impl ::std::fmt::Debug for PayloadZoneExitNotificationView<'_> {
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

impl ::protobuf::Serialize for PayloadZoneExitNotificationView<'_> {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    // SAFETY: `MINI_TABLE` is the one associated with `self.raw_msg()`.
    let encoded = unsafe {
      ::protobuf::__internal::runtime::wire::encode(self.raw_msg(),
          <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table())
    };
    encoded.map_err(|_| ::protobuf::SerializeError)
  }
}

impl ::std::default::Default for PayloadZoneExitNotificationView<'_> {
  fn default() -> PayloadZoneExitNotificationView<'static> {
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(::protobuf::__internal::runtime::ScratchSpace::zeroed_block()) };
    PayloadZoneExitNotificationView::new(::protobuf::__internal::Private, inner)
  }
}

#[allow(dead_code)]
impl<'msg> PayloadZoneExitNotificationView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::protobuf::__internal::Private, inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, PayloadZoneExitNotification>) -> Self {
    Self { inner, _phantom: ::std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  pub fn to_owned(&self) -> PayloadZoneExitNotification {
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
// - `PayloadZoneExitNotificationView` is `Sync` because it does not support mutation.
unsafe impl Sync for PayloadZoneExitNotificationView<'_> {}

// SAFETY:
// - `PayloadZoneExitNotificationView` is `Send` because while its alive a `PayloadZoneExitNotificationMut` cannot.
// - `PayloadZoneExitNotificationView` does not use thread-local data.
unsafe impl Send for PayloadZoneExitNotificationView<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for PayloadZoneExitNotificationView<'msg> {}
impl<'msg> ::protobuf::ViewProxy<'msg> for PayloadZoneExitNotificationView<'msg> {}

impl<'msg> ::protobuf::AsView for PayloadZoneExitNotificationView<'msg> {
  type Proxied = PayloadZoneExitNotification;
  fn as_view(&self) -> ::protobuf::View<'msg, PayloadZoneExitNotification> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PayloadZoneExitNotificationView<'msg> {
  fn into_view<'shorter>(self) -> PayloadZoneExitNotificationView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<PayloadZoneExitNotification> for PayloadZoneExitNotificationView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> PayloadZoneExitNotification {
    let mut dst = PayloadZoneExitNotification::new();
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

impl<'msg> ::protobuf::IntoProxied<PayloadZoneExitNotification> for PayloadZoneExitNotificationMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> PayloadZoneExitNotification {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::UpbTypeConversions for PayloadZoneExitNotification {
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
        PayloadZoneExitNotificationView::new(::protobuf::__internal::Private, inner)
    }

    unsafe fn from_message_mut<'msg>(msg: ::protobuf::__internal::runtime::RawMessage, arena: &'msg ::protobuf::__internal::runtime::Arena)
        -> PayloadZoneExitNotificationMut<'msg> {
        let inner = unsafe { ::protobuf::__internal::runtime::MessageMutInner::<'msg, PayloadZoneExitNotification>::wrap_raw(msg, arena) };
        PayloadZoneExitNotificationMut::new(::protobuf::__internal::Private, inner)
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct PayloadZoneExitNotificationMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, PayloadZoneExitNotification>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PayloadZoneExitNotificationMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for PayloadZoneExitNotificationMut<'msg> {
  type Message = PayloadZoneExitNotification;
}

impl ::std::fmt::Debug for PayloadZoneExitNotificationMut<'_> {
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

impl ::protobuf::Serialize for PayloadZoneExitNotificationMut<'_> {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

#[allow(dead_code)]
impl<'msg> PayloadZoneExitNotificationMut<'msg> {
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
  pub fn new(_private: ::protobuf::__internal::Private, inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, PayloadZoneExitNotification>) -> Self {
    Self { inner }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, PayloadZoneExitNotification> {
    self.inner
  }

  pub fn to_owned(&self) -> PayloadZoneExitNotification {
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
// - `PayloadZoneExitNotificationMut` does not perform any shared mutation.
// - `PayloadZoneExitNotificationMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for PayloadZoneExitNotificationMut<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for PayloadZoneExitNotificationMut<'msg> {}
impl<'msg> ::protobuf::MutProxy<'msg> for PayloadZoneExitNotificationMut<'msg> {}

impl<'msg> ::protobuf::AsView for PayloadZoneExitNotificationMut<'msg> {
  type Proxied = PayloadZoneExitNotification;
  fn as_view(&self) -> ::protobuf::View<'_, PayloadZoneExitNotification> {
    PayloadZoneExitNotificationView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner.clone()),
      _phantom: ::std::marker::PhantomData
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PayloadZoneExitNotificationMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, PayloadZoneExitNotification>
  where
      'msg: 'shorter {
    PayloadZoneExitNotificationView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner.clone()),
      _phantom: ::std::marker::PhantomData
    }
  }
}

impl<'msg> ::protobuf::AsMut for PayloadZoneExitNotificationMut<'msg> {
  type MutProxied = PayloadZoneExitNotification;
  fn as_mut(&mut self) -> PayloadZoneExitNotificationMut<'msg> {
    PayloadZoneExitNotificationMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for PayloadZoneExitNotificationMut<'msg> {
  fn into_mut<'shorter>(self) -> PayloadZoneExitNotificationMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl PayloadZoneExitNotification {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, PayloadZoneExitNotification> {
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

  pub fn as_view(&self) -> PayloadZoneExitNotificationView {
    PayloadZoneExitNotificationView::new(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner))
  }

  pub fn as_mut(&mut self) -> PayloadZoneExitNotificationMut {
    let inner = ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner);
    PayloadZoneExitNotificationMut::new(::protobuf::__internal::Private, inner)
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

}  // impl PayloadZoneExitNotification

impl ::std::ops::Drop for PayloadZoneExitNotification {
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for PayloadZoneExitNotification {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for PayloadZoneExitNotification {
  type Proxied = Self;
  fn as_view(&self) -> PayloadZoneExitNotificationView {
    self.as_view()
  }
}

impl ::protobuf::AsMut for PayloadZoneExitNotification {
  type MutProxied = Self;
  fn as_mut(&mut self) -> PayloadZoneExitNotificationMut {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for PayloadZoneExitNotification {
  fn mini_table() -> *const ::protobuf::__internal::runtime::upb_MiniTable {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTablePtr> =
        ::std::sync::OnceLock::new();
    ONCE_LOCK.get_or_init(|| unsafe {
      super::Proto__PayloadZoneExitNotification_msg_init.0 =
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
          super::Proto__PayloadZoneExitNotification_msg_init.0,
          submessages.as_ptr() as *const *const ::protobuf::__internal::runtime::upb_MiniTable,
          submessages.len(), subenums.as_ptr(), subenums.len()));
      ::protobuf::__internal::runtime::MiniTablePtr(super::Proto__PayloadZoneExitNotification_msg_init.0)
    }).0
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for PayloadZoneExitNotification {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for PayloadZoneExitNotificationView<'_> {
  #[inline(always)]
  fn mini_table() -> *const ::protobuf::__internal::runtime::upb_MiniTable {
    <PayloadZoneExitNotification as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for PayloadZoneExitNotificationMut<'_> {
  #[inline(always)]
  fn mini_table() -> *const ::protobuf::__internal::runtime::upb_MiniTable {
    <PayloadZoneExitNotification as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for PayloadZoneExitNotification {
  type Msg = PayloadZoneExitNotification;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PayloadZoneExitNotification> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PayloadZoneExitNotification {
  type Msg = PayloadZoneExitNotification;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PayloadZoneExitNotification> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for PayloadZoneExitNotificationMut<'_> {
  type Msg = PayloadZoneExitNotification;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PayloadZoneExitNotification> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PayloadZoneExitNotificationMut<'_> {
  type Msg = PayloadZoneExitNotification;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PayloadZoneExitNotification> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PayloadZoneExitNotificationView<'_> {
  type Msg = PayloadZoneExitNotification;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PayloadZoneExitNotification> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for PayloadZoneExitNotificationMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}


// upb kernel doesn't support any owned message or message mut interop.
impl ::protobuf::OwnedMessageInterop for PayloadZoneExitNotification {}
impl<'a> ::protobuf::MessageMutInterop<'a> for PayloadZoneExitNotificationMut<'a> {}

impl<'a> ::protobuf::MessageViewInterop<'a> for PayloadZoneExitNotificationView<'a> {
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

