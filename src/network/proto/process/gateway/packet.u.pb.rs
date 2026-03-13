const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.32.1-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut Proto__Packet_msg_init: ::protobuf::__internal::runtime::MiniTablePtr =
    ::protobuf::__internal::runtime::MiniTablePtr(::std::ptr::null_mut());
#[allow(non_camel_case_types)]
pub struct Packet {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Packet>
}

impl ::protobuf::Message for Packet {}

impl ::std::default::Default for Packet {
  fn default() -> Self {
    Self::new()
  }
}

impl ::protobuf::Parse for Packet {
  fn parse(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    Self::parse(serialized)
  }

  fn parse_dont_enforce_required(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    Self::parse_dont_enforce_required(serialized)
  }
}

impl ::std::fmt::Debug for Packet {
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

impl ::protobuf::Serialize for Packet {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

// SAFETY:
// - `Packet` is `Sync` because it does not implement interior mutability.
//    Neither does `PacketMut`.
unsafe impl Sync for Packet {}

// SAFETY:
// - `Packet` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for Packet {}

impl ::protobuf::Proxied for Packet {
  type View<'msg> = PacketView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Packet {}

impl ::protobuf::MutProxied for Packet {
  type Mut<'msg> = PacketMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct PacketView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Packet>,
  _phantom: ::std::marker::PhantomData<&'msg ()>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PacketView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for PacketView<'msg> {
  type Message = Packet;
}

impl ::std::fmt::Debug for PacketView<'_> {
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

impl ::protobuf::Serialize for PacketView<'_> {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    // SAFETY: `MINI_TABLE` is the one associated with `self.raw_msg()`.
    let encoded = unsafe {
      ::protobuf::__internal::runtime::wire::encode(self.raw_msg(),
          <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table())
    };
    encoded.map_err(|_| ::protobuf::SerializeError)
  }
}

impl ::std::default::Default for PacketView<'_> {
  fn default() -> PacketView<'static> {
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(::protobuf::__internal::runtime::ScratchSpace::zeroed_block()) };
    PacketView::new(::protobuf::__internal::Private, inner)
  }
}

#[allow(dead_code)]
impl<'msg> PacketView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::protobuf::__internal::Private, inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Packet>) -> Self {
    Self { inner, _phantom: ::std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  pub fn to_owned(&self) -> Packet {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // login_request: optional message Proto.PayloadLoginRequest
  pub fn has_login_request(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn login_request_opt(self) -> ::protobuf::Optional<super::PayloadLoginRequestView<'msg>> {
        ::protobuf::Optional::new(self.login_request(), self.has_login_request())
  }
  pub fn login_request(self) -> super::PayloadLoginRequestView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    let raw = submsg.map(|ptr| ptr.raw()).unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    super::PayloadLoginRequestView::new(::protobuf::__internal::Private, inner)
  }

  // login_result: optional message Proto.PayloadLoginResult
  pub fn has_login_result(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn login_result_opt(self) -> ::protobuf::Optional<super::PayloadLoginResultView<'msg>> {
        ::protobuf::Optional::new(self.login_result(), self.has_login_result())
  }
  pub fn login_result(self) -> super::PayloadLoginResultView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    let raw = submsg.map(|ptr| ptr.raw()).unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    super::PayloadLoginResultView::new(::protobuf::__internal::Private, inner)
  }

  // login_notification: optional message Proto.PayloadLoginNotification
  pub fn has_login_notification(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn login_notification_opt(self) -> ::protobuf::Optional<super::PayloadLoginNotificationView<'msg>> {
        ::protobuf::Optional::new(self.login_notification(), self.has_login_notification())
  }
  pub fn login_notification(self) -> super::PayloadLoginNotificationView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    let raw = submsg.map(|ptr| ptr.raw()).unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    super::PayloadLoginNotificationView::new(::protobuf::__internal::Private, inner)
  }

  // logout_request: optional message Proto.PayloadLogoutRequest
  pub fn has_logout_request(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn logout_request_opt(self) -> ::protobuf::Optional<super::PayloadLogoutRequestView<'msg>> {
        ::protobuf::Optional::new(self.logout_request(), self.has_logout_request())
  }
  pub fn logout_request(self) -> super::PayloadLogoutRequestView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    let raw = submsg.map(|ptr| ptr.raw()).unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    super::PayloadLogoutRequestView::new(::protobuf::__internal::Private, inner)
  }

  // logout_response: optional message Proto.PayloadLogoutResponse
  pub fn has_logout_response(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn logout_response_opt(self) -> ::protobuf::Optional<super::PayloadLogoutResponseView<'msg>> {
        ::protobuf::Optional::new(self.logout_response(), self.has_logout_response())
  }
  pub fn logout_response(self) -> super::PayloadLogoutResponseView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    let raw = submsg.map(|ptr| ptr.raw()).unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    super::PayloadLogoutResponseView::new(::protobuf::__internal::Private, inner)
  }

  // logout_notification: optional message Proto.PayloadLogoutNotification
  pub fn has_logout_notification(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn logout_notification_opt(self) -> ::protobuf::Optional<super::PayloadLogoutNotificationView<'msg>> {
        ::protobuf::Optional::new(self.logout_notification(), self.has_logout_notification())
  }
  pub fn logout_notification(self) -> super::PayloadLogoutNotificationView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    let raw = submsg.map(|ptr| ptr.raw()).unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    super::PayloadLogoutNotificationView::new(::protobuf::__internal::Private, inner)
  }

  // transform_sync: optional message Proto.PayloadTransformSync
  pub fn has_transform_sync(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn transform_sync_opt(self) -> ::protobuf::Optional<super::PayloadTransformSyncView<'msg>> {
        ::protobuf::Optional::new(self.transform_sync(), self.has_transform_sync())
  }
  pub fn transform_sync(self) -> super::PayloadTransformSyncView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    let raw = submsg.map(|ptr| ptr.raw()).unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    super::PayloadTransformSyncView::new(::protobuf::__internal::Private, inner)
  }

  // play_action: optional message Proto.PayloadPlayAction
  pub fn has_play_action(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn play_action_opt(self) -> ::protobuf::Optional<super::PayloadPlayActionView<'msg>> {
        ::protobuf::Optional::new(self.play_action(), self.has_play_action())
  }
  pub fn play_action(self) -> super::PayloadPlayActionView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(7)
    };
    let raw = submsg.map(|ptr| ptr.raw()).unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    super::PayloadPlayActionView::new(::protobuf::__internal::Private, inner)
  }

  // entity_damaged: optional message Proto.PayloadEntityDamaged
  pub fn has_entity_damaged(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(8)
    }
  }
  pub fn entity_damaged_opt(self) -> ::protobuf::Optional<super::PayloadEntityDamagedView<'msg>> {
        ::protobuf::Optional::new(self.entity_damaged(), self.has_entity_damaged())
  }
  pub fn entity_damaged(self) -> super::PayloadEntityDamagedView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(8)
    };
    let raw = submsg.map(|ptr| ptr.raw()).unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    super::PayloadEntityDamagedView::new(::protobuf::__internal::Private, inner)
  }

  // enemy_spawn: optional message Proto.PayloadEnemySpawn
  pub fn has_enemy_spawn(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(9)
    }
  }
  pub fn enemy_spawn_opt(self) -> ::protobuf::Optional<super::PayloadEnemySpawnView<'msg>> {
        ::protobuf::Optional::new(self.enemy_spawn(), self.has_enemy_spawn())
  }
  pub fn enemy_spawn(self) -> super::PayloadEnemySpawnView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(9)
    };
    let raw = submsg.map(|ptr| ptr.raw()).unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    super::PayloadEnemySpawnView::new(::protobuf::__internal::Private, inner)
  }

  // enemy_despawn: optional message Proto.PayloadEnemyDespawn
  pub fn has_enemy_despawn(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(10)
    }
  }
  pub fn enemy_despawn_opt(self) -> ::protobuf::Optional<super::PayloadEnemyDespawnView<'msg>> {
        ::protobuf::Optional::new(self.enemy_despawn(), self.has_enemy_despawn())
  }
  pub fn enemy_despawn(self) -> super::PayloadEnemyDespawnView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(10)
    };
    let raw = submsg.map(|ptr| ptr.raw()).unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    super::PayloadEnemyDespawnView::new(::protobuf::__internal::Private, inner)
  }

  // text_message: optional message Proto.PayloadTextMessage
  pub fn has_text_message(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(11)
    }
  }
  pub fn text_message_opt(self) -> ::protobuf::Optional<super::PayloadTextMessageView<'msg>> {
        ::protobuf::Optional::new(self.text_message(), self.has_text_message())
  }
  pub fn text_message(self) -> super::PayloadTextMessageView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(11)
    };
    let raw = submsg.map(|ptr| ptr.raw()).unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    super::PayloadTextMessageView::new(::protobuf::__internal::Private, inner)
  }

  // system_message: optional message Proto.PayloadSystemMessage
  pub fn has_system_message(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(12)
    }
  }
  pub fn system_message_opt(self) -> ::protobuf::Optional<super::PayloadSystemMessageView<'msg>> {
        ::protobuf::Optional::new(self.system_message(), self.has_system_message())
  }
  pub fn system_message(self) -> super::PayloadSystemMessageView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(12)
    };
    let raw = submsg.map(|ptr| ptr.raw()).unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    super::PayloadSystemMessageView::new(::protobuf::__internal::Private, inner)
  }

  // zone_enter_request: optional message Proto.PayloadZoneEnterRequest
  pub fn has_zone_enter_request(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(13)
    }
  }
  pub fn zone_enter_request_opt(self) -> ::protobuf::Optional<super::PayloadZoneEnterRequestView<'msg>> {
        ::protobuf::Optional::new(self.zone_enter_request(), self.has_zone_enter_request())
  }
  pub fn zone_enter_request(self) -> super::PayloadZoneEnterRequestView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(13)
    };
    let raw = submsg.map(|ptr| ptr.raw()).unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    super::PayloadZoneEnterRequestView::new(::protobuf::__internal::Private, inner)
  }

  // zone_enter_response: optional message Proto.PayloadZoneEnterResponse
  pub fn has_zone_enter_response(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(14)
    }
  }
  pub fn zone_enter_response_opt(self) -> ::protobuf::Optional<super::PayloadZoneEnterResponseView<'msg>> {
        ::protobuf::Optional::new(self.zone_enter_response(), self.has_zone_enter_response())
  }
  pub fn zone_enter_response(self) -> super::PayloadZoneEnterResponseView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(14)
    };
    let raw = submsg.map(|ptr| ptr.raw()).unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    super::PayloadZoneEnterResponseView::new(::protobuf::__internal::Private, inner)
  }

  // zone_exit: optional message Proto.PayloadZoneExit
  pub fn has_zone_exit(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(15)
    }
  }
  pub fn zone_exit_opt(self) -> ::protobuf::Optional<super::PayloadZoneExitView<'msg>> {
        ::protobuf::Optional::new(self.zone_exit(), self.has_zone_exit())
  }
  pub fn zone_exit(self) -> super::PayloadZoneExitView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(15)
    };
    let raw = submsg.map(|ptr| ptr.raw()).unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    super::PayloadZoneExitView::new(::protobuf::__internal::Private, inner)
  }

  pub fn message(self) -> super::packet::MessageOneof<'msg> {
    match self.message_case() {
      super::packet::MessageCase::LoginRequest =>
          super::packet::MessageOneof::LoginRequest(self.login_request()),
      super::packet::MessageCase::LoginResult =>
          super::packet::MessageOneof::LoginResult(self.login_result()),
      super::packet::MessageCase::LoginNotification =>
          super::packet::MessageOneof::LoginNotification(self.login_notification()),
      super::packet::MessageCase::LogoutRequest =>
          super::packet::MessageOneof::LogoutRequest(self.logout_request()),
      super::packet::MessageCase::LogoutResponse =>
          super::packet::MessageOneof::LogoutResponse(self.logout_response()),
      super::packet::MessageCase::LogoutNotification =>
          super::packet::MessageOneof::LogoutNotification(self.logout_notification()),
      super::packet::MessageCase::TransformSync =>
          super::packet::MessageOneof::TransformSync(self.transform_sync()),
      super::packet::MessageCase::PlayAction =>
          super::packet::MessageOneof::PlayAction(self.play_action()),
      super::packet::MessageCase::EntityDamaged =>
          super::packet::MessageOneof::EntityDamaged(self.entity_damaged()),
      super::packet::MessageCase::EnemySpawn =>
          super::packet::MessageOneof::EnemySpawn(self.enemy_spawn()),
      super::packet::MessageCase::EnemyDespawn =>
          super::packet::MessageOneof::EnemyDespawn(self.enemy_despawn()),
      super::packet::MessageCase::TextMessage =>
          super::packet::MessageOneof::TextMessage(self.text_message()),
      super::packet::MessageCase::SystemMessage =>
          super::packet::MessageOneof::SystemMessage(self.system_message()),
      super::packet::MessageCase::ZoneEnterRequest =>
          super::packet::MessageOneof::ZoneEnterRequest(self.zone_enter_request()),
      super::packet::MessageCase::ZoneEnterResponse =>
          super::packet::MessageOneof::ZoneEnterResponse(self.zone_enter_response()),
      super::packet::MessageCase::ZoneExit =>
          super::packet::MessageOneof::ZoneExit(self.zone_exit()),
      _ => super::packet::MessageOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn message_case(self) -> super::packet::MessageCase {
    let field_num = unsafe {
      let f = ::protobuf::__internal::runtime::upb_MiniTable_GetFieldByIndex(
          <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
          0);
      ::protobuf::__internal::runtime::upb_Message_WhichOneofFieldNumber(
            self.raw_msg(), f)
    };
    unsafe {
      super::packet::MessageCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `PacketView` is `Sync` because it does not support mutation.
unsafe impl Sync for PacketView<'_> {}

// SAFETY:
// - `PacketView` is `Send` because while its alive a `PacketMut` cannot.
// - `PacketView` does not use thread-local data.
unsafe impl Send for PacketView<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for PacketView<'msg> {}
impl<'msg> ::protobuf::ViewProxy<'msg> for PacketView<'msg> {}

impl<'msg> ::protobuf::AsView for PacketView<'msg> {
  type Proxied = Packet;
  fn as_view(&self) -> ::protobuf::View<'msg, Packet> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PacketView<'msg> {
  fn into_view<'shorter>(self) -> PacketView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Packet> for PacketView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Packet {
    let mut dst = Packet::new();
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

impl<'msg> ::protobuf::IntoProxied<Packet> for PacketMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Packet {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::UpbTypeConversions for Packet {
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
        PacketView::new(::protobuf::__internal::Private, inner)
    }

    unsafe fn from_message_mut<'msg>(msg: ::protobuf::__internal::runtime::RawMessage, arena: &'msg ::protobuf::__internal::runtime::Arena)
        -> PacketMut<'msg> {
        let inner = unsafe { ::protobuf::__internal::runtime::MessageMutInner::<'msg, Packet>::wrap_raw(msg, arena) };
        PacketMut::new(::protobuf::__internal::Private, inner)
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct PacketMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Packet>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PacketMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for PacketMut<'msg> {
  type Message = Packet;
}

impl ::std::fmt::Debug for PacketMut<'_> {
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

impl ::protobuf::Serialize for PacketMut<'_> {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

#[allow(dead_code)]
impl<'msg> PacketMut<'msg> {
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
  pub fn new(_private: ::protobuf::__internal::Private, inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Packet>) -> Self {
    Self { inner }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Packet> {
    self.inner
  }

  pub fn to_owned(&self) -> Packet {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  fn arena(&mut self) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }

  // login_request: optional message Proto.PayloadLoginRequest
  pub fn has_login_request(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_login_request(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn login_request_opt(&self) -> ::protobuf::Optional<super::PayloadLoginRequestView<'_>> {
        ::protobuf::Optional::new(self.login_request(), self.has_login_request())
  }
  pub fn login_request(&self) -> super::PayloadLoginRequestView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    let raw = submsg.map(|ptr| ptr.raw()).unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    super::PayloadLoginRequestView::new(::protobuf::__internal::Private, inner)
  }
  pub fn login_request_mut(&mut self) -> super::PayloadLoginRequestMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         0, self.arena()
       ).unwrap()
     };
     super::PayloadLoginRequestMut::from_parent(
       ::protobuf::__internal::Private,
       self.as_message_mut_inner(::protobuf::__internal::Private),
       ptr.raw())
  }
  pub fn set_login_request(&mut self,
    val: impl ::protobuf::IntoProxied<super::PayloadLoginRequest>) {

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
        0, child_ptr
      );
    }
  }

  // login_result: optional message Proto.PayloadLoginResult
  pub fn has_login_result(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_login_result(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn login_result_opt(&self) -> ::protobuf::Optional<super::PayloadLoginResultView<'_>> {
        ::protobuf::Optional::new(self.login_result(), self.has_login_result())
  }
  pub fn login_result(&self) -> super::PayloadLoginResultView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    let raw = submsg.map(|ptr| ptr.raw()).unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    super::PayloadLoginResultView::new(::protobuf::__internal::Private, inner)
  }
  pub fn login_result_mut(&mut self) -> super::PayloadLoginResultMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         1, self.arena()
       ).unwrap()
     };
     super::PayloadLoginResultMut::from_parent(
       ::protobuf::__internal::Private,
       self.as_message_mut_inner(::protobuf::__internal::Private),
       ptr.raw())
  }
  pub fn set_login_result(&mut self,
    val: impl ::protobuf::IntoProxied<super::PayloadLoginResult>) {

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
        1, child_ptr
      );
    }
  }

  // login_notification: optional message Proto.PayloadLoginNotification
  pub fn has_login_notification(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_login_notification(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn login_notification_opt(&self) -> ::protobuf::Optional<super::PayloadLoginNotificationView<'_>> {
        ::protobuf::Optional::new(self.login_notification(), self.has_login_notification())
  }
  pub fn login_notification(&self) -> super::PayloadLoginNotificationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    let raw = submsg.map(|ptr| ptr.raw()).unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    super::PayloadLoginNotificationView::new(::protobuf::__internal::Private, inner)
  }
  pub fn login_notification_mut(&mut self) -> super::PayloadLoginNotificationMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         2, self.arena()
       ).unwrap()
     };
     super::PayloadLoginNotificationMut::from_parent(
       ::protobuf::__internal::Private,
       self.as_message_mut_inner(::protobuf::__internal::Private),
       ptr.raw())
  }
  pub fn set_login_notification(&mut self,
    val: impl ::protobuf::IntoProxied<super::PayloadLoginNotification>) {

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

  // logout_request: optional message Proto.PayloadLogoutRequest
  pub fn has_logout_request(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_logout_request(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn logout_request_opt(&self) -> ::protobuf::Optional<super::PayloadLogoutRequestView<'_>> {
        ::protobuf::Optional::new(self.logout_request(), self.has_logout_request())
  }
  pub fn logout_request(&self) -> super::PayloadLogoutRequestView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    let raw = submsg.map(|ptr| ptr.raw()).unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    super::PayloadLogoutRequestView::new(::protobuf::__internal::Private, inner)
  }
  pub fn logout_request_mut(&mut self) -> super::PayloadLogoutRequestMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         3, self.arena()
       ).unwrap()
     };
     super::PayloadLogoutRequestMut::from_parent(
       ::protobuf::__internal::Private,
       self.as_message_mut_inner(::protobuf::__internal::Private),
       ptr.raw())
  }
  pub fn set_logout_request(&mut self,
    val: impl ::protobuf::IntoProxied<super::PayloadLogoutRequest>) {

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
        3, child_ptr
      );
    }
  }

  // logout_response: optional message Proto.PayloadLogoutResponse
  pub fn has_logout_response(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_logout_response(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn logout_response_opt(&self) -> ::protobuf::Optional<super::PayloadLogoutResponseView<'_>> {
        ::protobuf::Optional::new(self.logout_response(), self.has_logout_response())
  }
  pub fn logout_response(&self) -> super::PayloadLogoutResponseView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    let raw = submsg.map(|ptr| ptr.raw()).unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    super::PayloadLogoutResponseView::new(::protobuf::__internal::Private, inner)
  }
  pub fn logout_response_mut(&mut self) -> super::PayloadLogoutResponseMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         4, self.arena()
       ).unwrap()
     };
     super::PayloadLogoutResponseMut::from_parent(
       ::protobuf::__internal::Private,
       self.as_message_mut_inner(::protobuf::__internal::Private),
       ptr.raw())
  }
  pub fn set_logout_response(&mut self,
    val: impl ::protobuf::IntoProxied<super::PayloadLogoutResponse>) {

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
        4, child_ptr
      );
    }
  }

  // logout_notification: optional message Proto.PayloadLogoutNotification
  pub fn has_logout_notification(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_logout_notification(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn logout_notification_opt(&self) -> ::protobuf::Optional<super::PayloadLogoutNotificationView<'_>> {
        ::protobuf::Optional::new(self.logout_notification(), self.has_logout_notification())
  }
  pub fn logout_notification(&self) -> super::PayloadLogoutNotificationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    let raw = submsg.map(|ptr| ptr.raw()).unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    super::PayloadLogoutNotificationView::new(::protobuf::__internal::Private, inner)
  }
  pub fn logout_notification_mut(&mut self) -> super::PayloadLogoutNotificationMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         5, self.arena()
       ).unwrap()
     };
     super::PayloadLogoutNotificationMut::from_parent(
       ::protobuf::__internal::Private,
       self.as_message_mut_inner(::protobuf::__internal::Private),
       ptr.raw())
  }
  pub fn set_logout_notification(&mut self,
    val: impl ::protobuf::IntoProxied<super::PayloadLogoutNotification>) {

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
        5, child_ptr
      );
    }
  }

  // transform_sync: optional message Proto.PayloadTransformSync
  pub fn has_transform_sync(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn clear_transform_sync(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        6
      );
    }
  }
  pub fn transform_sync_opt(&self) -> ::protobuf::Optional<super::PayloadTransformSyncView<'_>> {
        ::protobuf::Optional::new(self.transform_sync(), self.has_transform_sync())
  }
  pub fn transform_sync(&self) -> super::PayloadTransformSyncView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    let raw = submsg.map(|ptr| ptr.raw()).unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    super::PayloadTransformSyncView::new(::protobuf::__internal::Private, inner)
  }
  pub fn transform_sync_mut(&mut self) -> super::PayloadTransformSyncMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         6, self.arena()
       ).unwrap()
     };
     super::PayloadTransformSyncMut::from_parent(
       ::protobuf::__internal::Private,
       self.as_message_mut_inner(::protobuf::__internal::Private),
       ptr.raw())
  }
  pub fn set_transform_sync(&mut self,
    val: impl ::protobuf::IntoProxied<super::PayloadTransformSync>) {

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
        6, child_ptr
      );
    }
  }

  // play_action: optional message Proto.PayloadPlayAction
  pub fn has_play_action(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn clear_play_action(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        7
      );
    }
  }
  pub fn play_action_opt(&self) -> ::protobuf::Optional<super::PayloadPlayActionView<'_>> {
        ::protobuf::Optional::new(self.play_action(), self.has_play_action())
  }
  pub fn play_action(&self) -> super::PayloadPlayActionView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(7)
    };
    let raw = submsg.map(|ptr| ptr.raw()).unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    super::PayloadPlayActionView::new(::protobuf::__internal::Private, inner)
  }
  pub fn play_action_mut(&mut self) -> super::PayloadPlayActionMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         7, self.arena()
       ).unwrap()
     };
     super::PayloadPlayActionMut::from_parent(
       ::protobuf::__internal::Private,
       self.as_message_mut_inner(::protobuf::__internal::Private),
       ptr.raw())
  }
  pub fn set_play_action(&mut self,
    val: impl ::protobuf::IntoProxied<super::PayloadPlayAction>) {

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
        7, child_ptr
      );
    }
  }

  // entity_damaged: optional message Proto.PayloadEntityDamaged
  pub fn has_entity_damaged(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(8)
    }
  }
  pub fn clear_entity_damaged(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        8
      );
    }
  }
  pub fn entity_damaged_opt(&self) -> ::protobuf::Optional<super::PayloadEntityDamagedView<'_>> {
        ::protobuf::Optional::new(self.entity_damaged(), self.has_entity_damaged())
  }
  pub fn entity_damaged(&self) -> super::PayloadEntityDamagedView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(8)
    };
    let raw = submsg.map(|ptr| ptr.raw()).unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    super::PayloadEntityDamagedView::new(::protobuf::__internal::Private, inner)
  }
  pub fn entity_damaged_mut(&mut self) -> super::PayloadEntityDamagedMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         8, self.arena()
       ).unwrap()
     };
     super::PayloadEntityDamagedMut::from_parent(
       ::protobuf::__internal::Private,
       self.as_message_mut_inner(::protobuf::__internal::Private),
       ptr.raw())
  }
  pub fn set_entity_damaged(&mut self,
    val: impl ::protobuf::IntoProxied<super::PayloadEntityDamaged>) {

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
        8, child_ptr
      );
    }
  }

  // enemy_spawn: optional message Proto.PayloadEnemySpawn
  pub fn has_enemy_spawn(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(9)
    }
  }
  pub fn clear_enemy_spawn(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        9
      );
    }
  }
  pub fn enemy_spawn_opt(&self) -> ::protobuf::Optional<super::PayloadEnemySpawnView<'_>> {
        ::protobuf::Optional::new(self.enemy_spawn(), self.has_enemy_spawn())
  }
  pub fn enemy_spawn(&self) -> super::PayloadEnemySpawnView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(9)
    };
    let raw = submsg.map(|ptr| ptr.raw()).unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    super::PayloadEnemySpawnView::new(::protobuf::__internal::Private, inner)
  }
  pub fn enemy_spawn_mut(&mut self) -> super::PayloadEnemySpawnMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         9, self.arena()
       ).unwrap()
     };
     super::PayloadEnemySpawnMut::from_parent(
       ::protobuf::__internal::Private,
       self.as_message_mut_inner(::protobuf::__internal::Private),
       ptr.raw())
  }
  pub fn set_enemy_spawn(&mut self,
    val: impl ::protobuf::IntoProxied<super::PayloadEnemySpawn>) {

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
        9, child_ptr
      );
    }
  }

  // enemy_despawn: optional message Proto.PayloadEnemyDespawn
  pub fn has_enemy_despawn(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(10)
    }
  }
  pub fn clear_enemy_despawn(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        10
      );
    }
  }
  pub fn enemy_despawn_opt(&self) -> ::protobuf::Optional<super::PayloadEnemyDespawnView<'_>> {
        ::protobuf::Optional::new(self.enemy_despawn(), self.has_enemy_despawn())
  }
  pub fn enemy_despawn(&self) -> super::PayloadEnemyDespawnView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(10)
    };
    let raw = submsg.map(|ptr| ptr.raw()).unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    super::PayloadEnemyDespawnView::new(::protobuf::__internal::Private, inner)
  }
  pub fn enemy_despawn_mut(&mut self) -> super::PayloadEnemyDespawnMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         10, self.arena()
       ).unwrap()
     };
     super::PayloadEnemyDespawnMut::from_parent(
       ::protobuf::__internal::Private,
       self.as_message_mut_inner(::protobuf::__internal::Private),
       ptr.raw())
  }
  pub fn set_enemy_despawn(&mut self,
    val: impl ::protobuf::IntoProxied<super::PayloadEnemyDespawn>) {

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
        10, child_ptr
      );
    }
  }

  // text_message: optional message Proto.PayloadTextMessage
  pub fn has_text_message(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(11)
    }
  }
  pub fn clear_text_message(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        11
      );
    }
  }
  pub fn text_message_opt(&self) -> ::protobuf::Optional<super::PayloadTextMessageView<'_>> {
        ::protobuf::Optional::new(self.text_message(), self.has_text_message())
  }
  pub fn text_message(&self) -> super::PayloadTextMessageView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(11)
    };
    let raw = submsg.map(|ptr| ptr.raw()).unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    super::PayloadTextMessageView::new(::protobuf::__internal::Private, inner)
  }
  pub fn text_message_mut(&mut self) -> super::PayloadTextMessageMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         11, self.arena()
       ).unwrap()
     };
     super::PayloadTextMessageMut::from_parent(
       ::protobuf::__internal::Private,
       self.as_message_mut_inner(::protobuf::__internal::Private),
       ptr.raw())
  }
  pub fn set_text_message(&mut self,
    val: impl ::protobuf::IntoProxied<super::PayloadTextMessage>) {

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
        11, child_ptr
      );
    }
  }

  // system_message: optional message Proto.PayloadSystemMessage
  pub fn has_system_message(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(12)
    }
  }
  pub fn clear_system_message(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        12
      );
    }
  }
  pub fn system_message_opt(&self) -> ::protobuf::Optional<super::PayloadSystemMessageView<'_>> {
        ::protobuf::Optional::new(self.system_message(), self.has_system_message())
  }
  pub fn system_message(&self) -> super::PayloadSystemMessageView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(12)
    };
    let raw = submsg.map(|ptr| ptr.raw()).unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    super::PayloadSystemMessageView::new(::protobuf::__internal::Private, inner)
  }
  pub fn system_message_mut(&mut self) -> super::PayloadSystemMessageMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         12, self.arena()
       ).unwrap()
     };
     super::PayloadSystemMessageMut::from_parent(
       ::protobuf::__internal::Private,
       self.as_message_mut_inner(::protobuf::__internal::Private),
       ptr.raw())
  }
  pub fn set_system_message(&mut self,
    val: impl ::protobuf::IntoProxied<super::PayloadSystemMessage>) {

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
        12, child_ptr
      );
    }
  }

  // zone_enter_request: optional message Proto.PayloadZoneEnterRequest
  pub fn has_zone_enter_request(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(13)
    }
  }
  pub fn clear_zone_enter_request(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        13
      );
    }
  }
  pub fn zone_enter_request_opt(&self) -> ::protobuf::Optional<super::PayloadZoneEnterRequestView<'_>> {
        ::protobuf::Optional::new(self.zone_enter_request(), self.has_zone_enter_request())
  }
  pub fn zone_enter_request(&self) -> super::PayloadZoneEnterRequestView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(13)
    };
    let raw = submsg.map(|ptr| ptr.raw()).unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    super::PayloadZoneEnterRequestView::new(::protobuf::__internal::Private, inner)
  }
  pub fn zone_enter_request_mut(&mut self) -> super::PayloadZoneEnterRequestMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         13, self.arena()
       ).unwrap()
     };
     super::PayloadZoneEnterRequestMut::from_parent(
       ::protobuf::__internal::Private,
       self.as_message_mut_inner(::protobuf::__internal::Private),
       ptr.raw())
  }
  pub fn set_zone_enter_request(&mut self,
    val: impl ::protobuf::IntoProxied<super::PayloadZoneEnterRequest>) {

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
        13, child_ptr
      );
    }
  }

  // zone_enter_response: optional message Proto.PayloadZoneEnterResponse
  pub fn has_zone_enter_response(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(14)
    }
  }
  pub fn clear_zone_enter_response(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        14
      );
    }
  }
  pub fn zone_enter_response_opt(&self) -> ::protobuf::Optional<super::PayloadZoneEnterResponseView<'_>> {
        ::protobuf::Optional::new(self.zone_enter_response(), self.has_zone_enter_response())
  }
  pub fn zone_enter_response(&self) -> super::PayloadZoneEnterResponseView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(14)
    };
    let raw = submsg.map(|ptr| ptr.raw()).unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    super::PayloadZoneEnterResponseView::new(::protobuf::__internal::Private, inner)
  }
  pub fn zone_enter_response_mut(&mut self) -> super::PayloadZoneEnterResponseMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         14, self.arena()
       ).unwrap()
     };
     super::PayloadZoneEnterResponseMut::from_parent(
       ::protobuf::__internal::Private,
       self.as_message_mut_inner(::protobuf::__internal::Private),
       ptr.raw())
  }
  pub fn set_zone_enter_response(&mut self,
    val: impl ::protobuf::IntoProxied<super::PayloadZoneEnterResponse>) {

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
        14, child_ptr
      );
    }
  }

  // zone_exit: optional message Proto.PayloadZoneExit
  pub fn has_zone_exit(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(15)
    }
  }
  pub fn clear_zone_exit(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        15
      );
    }
  }
  pub fn zone_exit_opt(&self) -> ::protobuf::Optional<super::PayloadZoneExitView<'_>> {
        ::protobuf::Optional::new(self.zone_exit(), self.has_zone_exit())
  }
  pub fn zone_exit(&self) -> super::PayloadZoneExitView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(15)
    };
    let raw = submsg.map(|ptr| ptr.raw()).unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    super::PayloadZoneExitView::new(::protobuf::__internal::Private, inner)
  }
  pub fn zone_exit_mut(&mut self) -> super::PayloadZoneExitMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         15, self.arena()
       ).unwrap()
     };
     super::PayloadZoneExitMut::from_parent(
       ::protobuf::__internal::Private,
       self.as_message_mut_inner(::protobuf::__internal::Private),
       ptr.raw())
  }
  pub fn set_zone_exit(&mut self,
    val: impl ::protobuf::IntoProxied<super::PayloadZoneExit>) {

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
        15, child_ptr
      );
    }
  }

  pub fn message(&self) -> super::packet::MessageOneof<'_> {
    match &self.message_case() {
      super::packet::MessageCase::LoginRequest =>
          super::packet::MessageOneof::LoginRequest(self.login_request()),
      super::packet::MessageCase::LoginResult =>
          super::packet::MessageOneof::LoginResult(self.login_result()),
      super::packet::MessageCase::LoginNotification =>
          super::packet::MessageOneof::LoginNotification(self.login_notification()),
      super::packet::MessageCase::LogoutRequest =>
          super::packet::MessageOneof::LogoutRequest(self.logout_request()),
      super::packet::MessageCase::LogoutResponse =>
          super::packet::MessageOneof::LogoutResponse(self.logout_response()),
      super::packet::MessageCase::LogoutNotification =>
          super::packet::MessageOneof::LogoutNotification(self.logout_notification()),
      super::packet::MessageCase::TransformSync =>
          super::packet::MessageOneof::TransformSync(self.transform_sync()),
      super::packet::MessageCase::PlayAction =>
          super::packet::MessageOneof::PlayAction(self.play_action()),
      super::packet::MessageCase::EntityDamaged =>
          super::packet::MessageOneof::EntityDamaged(self.entity_damaged()),
      super::packet::MessageCase::EnemySpawn =>
          super::packet::MessageOneof::EnemySpawn(self.enemy_spawn()),
      super::packet::MessageCase::EnemyDespawn =>
          super::packet::MessageOneof::EnemyDespawn(self.enemy_despawn()),
      super::packet::MessageCase::TextMessage =>
          super::packet::MessageOneof::TextMessage(self.text_message()),
      super::packet::MessageCase::SystemMessage =>
          super::packet::MessageOneof::SystemMessage(self.system_message()),
      super::packet::MessageCase::ZoneEnterRequest =>
          super::packet::MessageOneof::ZoneEnterRequest(self.zone_enter_request()),
      super::packet::MessageCase::ZoneEnterResponse =>
          super::packet::MessageOneof::ZoneEnterResponse(self.zone_enter_response()),
      super::packet::MessageCase::ZoneExit =>
          super::packet::MessageOneof::ZoneExit(self.zone_exit()),
      _ => super::packet::MessageOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn message_case(&self) -> super::packet::MessageCase {
    let field_num = unsafe {
      let f = ::protobuf::__internal::runtime::upb_MiniTable_GetFieldByIndex(
          <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
          0);
      ::protobuf::__internal::runtime::upb_Message_WhichOneofFieldNumber(
            self.raw_msg(), f)
    };
    unsafe {
      super::packet::MessageCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `PacketMut` does not perform any shared mutation.
// - `PacketMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for PacketMut<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for PacketMut<'msg> {}
impl<'msg> ::protobuf::MutProxy<'msg> for PacketMut<'msg> {}

impl<'msg> ::protobuf::AsView for PacketMut<'msg> {
  type Proxied = Packet;
  fn as_view(&self) -> ::protobuf::View<'_, Packet> {
    PacketView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner.clone()),
      _phantom: ::std::marker::PhantomData
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PacketMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Packet>
  where
      'msg: 'shorter {
    PacketView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner.clone()),
      _phantom: ::std::marker::PhantomData
    }
  }
}

impl<'msg> ::protobuf::AsMut for PacketMut<'msg> {
  type MutProxied = Packet;
  fn as_mut(&mut self) -> PacketMut<'msg> {
    PacketMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for PacketMut<'msg> {
  fn into_mut<'shorter>(self) -> PacketMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Packet {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Packet> {
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

  pub fn as_view(&self) -> PacketView {
    PacketView::new(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner))
  }

  pub fn as_mut(&mut self) -> PacketMut {
    let inner = ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner);
    PacketMut::new(::protobuf::__internal::Private, inner)
  }

  // login_request: optional message Proto.PayloadLoginRequest
  pub fn has_login_request(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_login_request(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn login_request_opt(&self) -> ::protobuf::Optional<super::PayloadLoginRequestView<'_>> {
        ::protobuf::Optional::new(self.login_request(), self.has_login_request())
  }
  pub fn login_request(&self) -> super::PayloadLoginRequestView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    let raw = submsg.map(|ptr| ptr.raw()).unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    super::PayloadLoginRequestView::new(::protobuf::__internal::Private, inner)
  }
  pub fn login_request_mut(&mut self) -> super::PayloadLoginRequestMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         0, self.arena()
       ).unwrap()
     };
     super::PayloadLoginRequestMut::from_parent(
       ::protobuf::__internal::Private,
       self.as_message_mut_inner(::protobuf::__internal::Private),
       ptr.raw())
  }
  pub fn set_login_request(&mut self,
    val: impl ::protobuf::IntoProxied<super::PayloadLoginRequest>) {

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
        0, child_ptr
      );
    }
  }

  // login_result: optional message Proto.PayloadLoginResult
  pub fn has_login_result(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_login_result(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn login_result_opt(&self) -> ::protobuf::Optional<super::PayloadLoginResultView<'_>> {
        ::protobuf::Optional::new(self.login_result(), self.has_login_result())
  }
  pub fn login_result(&self) -> super::PayloadLoginResultView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    let raw = submsg.map(|ptr| ptr.raw()).unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    super::PayloadLoginResultView::new(::protobuf::__internal::Private, inner)
  }
  pub fn login_result_mut(&mut self) -> super::PayloadLoginResultMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         1, self.arena()
       ).unwrap()
     };
     super::PayloadLoginResultMut::from_parent(
       ::protobuf::__internal::Private,
       self.as_message_mut_inner(::protobuf::__internal::Private),
       ptr.raw())
  }
  pub fn set_login_result(&mut self,
    val: impl ::protobuf::IntoProxied<super::PayloadLoginResult>) {

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
        1, child_ptr
      );
    }
  }

  // login_notification: optional message Proto.PayloadLoginNotification
  pub fn has_login_notification(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_login_notification(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn login_notification_opt(&self) -> ::protobuf::Optional<super::PayloadLoginNotificationView<'_>> {
        ::protobuf::Optional::new(self.login_notification(), self.has_login_notification())
  }
  pub fn login_notification(&self) -> super::PayloadLoginNotificationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    let raw = submsg.map(|ptr| ptr.raw()).unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    super::PayloadLoginNotificationView::new(::protobuf::__internal::Private, inner)
  }
  pub fn login_notification_mut(&mut self) -> super::PayloadLoginNotificationMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         2, self.arena()
       ).unwrap()
     };
     super::PayloadLoginNotificationMut::from_parent(
       ::protobuf::__internal::Private,
       self.as_message_mut_inner(::protobuf::__internal::Private),
       ptr.raw())
  }
  pub fn set_login_notification(&mut self,
    val: impl ::protobuf::IntoProxied<super::PayloadLoginNotification>) {

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

  // logout_request: optional message Proto.PayloadLogoutRequest
  pub fn has_logout_request(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_logout_request(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn logout_request_opt(&self) -> ::protobuf::Optional<super::PayloadLogoutRequestView<'_>> {
        ::protobuf::Optional::new(self.logout_request(), self.has_logout_request())
  }
  pub fn logout_request(&self) -> super::PayloadLogoutRequestView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    let raw = submsg.map(|ptr| ptr.raw()).unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    super::PayloadLogoutRequestView::new(::protobuf::__internal::Private, inner)
  }
  pub fn logout_request_mut(&mut self) -> super::PayloadLogoutRequestMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         3, self.arena()
       ).unwrap()
     };
     super::PayloadLogoutRequestMut::from_parent(
       ::protobuf::__internal::Private,
       self.as_message_mut_inner(::protobuf::__internal::Private),
       ptr.raw())
  }
  pub fn set_logout_request(&mut self,
    val: impl ::protobuf::IntoProxied<super::PayloadLogoutRequest>) {

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
        3, child_ptr
      );
    }
  }

  // logout_response: optional message Proto.PayloadLogoutResponse
  pub fn has_logout_response(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_logout_response(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn logout_response_opt(&self) -> ::protobuf::Optional<super::PayloadLogoutResponseView<'_>> {
        ::protobuf::Optional::new(self.logout_response(), self.has_logout_response())
  }
  pub fn logout_response(&self) -> super::PayloadLogoutResponseView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    let raw = submsg.map(|ptr| ptr.raw()).unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    super::PayloadLogoutResponseView::new(::protobuf::__internal::Private, inner)
  }
  pub fn logout_response_mut(&mut self) -> super::PayloadLogoutResponseMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         4, self.arena()
       ).unwrap()
     };
     super::PayloadLogoutResponseMut::from_parent(
       ::protobuf::__internal::Private,
       self.as_message_mut_inner(::protobuf::__internal::Private),
       ptr.raw())
  }
  pub fn set_logout_response(&mut self,
    val: impl ::protobuf::IntoProxied<super::PayloadLogoutResponse>) {

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
        4, child_ptr
      );
    }
  }

  // logout_notification: optional message Proto.PayloadLogoutNotification
  pub fn has_logout_notification(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_logout_notification(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn logout_notification_opt(&self) -> ::protobuf::Optional<super::PayloadLogoutNotificationView<'_>> {
        ::protobuf::Optional::new(self.logout_notification(), self.has_logout_notification())
  }
  pub fn logout_notification(&self) -> super::PayloadLogoutNotificationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    let raw = submsg.map(|ptr| ptr.raw()).unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    super::PayloadLogoutNotificationView::new(::protobuf::__internal::Private, inner)
  }
  pub fn logout_notification_mut(&mut self) -> super::PayloadLogoutNotificationMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         5, self.arena()
       ).unwrap()
     };
     super::PayloadLogoutNotificationMut::from_parent(
       ::protobuf::__internal::Private,
       self.as_message_mut_inner(::protobuf::__internal::Private),
       ptr.raw())
  }
  pub fn set_logout_notification(&mut self,
    val: impl ::protobuf::IntoProxied<super::PayloadLogoutNotification>) {

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
        5, child_ptr
      );
    }
  }

  // transform_sync: optional message Proto.PayloadTransformSync
  pub fn has_transform_sync(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn clear_transform_sync(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        6
      );
    }
  }
  pub fn transform_sync_opt(&self) -> ::protobuf::Optional<super::PayloadTransformSyncView<'_>> {
        ::protobuf::Optional::new(self.transform_sync(), self.has_transform_sync())
  }
  pub fn transform_sync(&self) -> super::PayloadTransformSyncView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    let raw = submsg.map(|ptr| ptr.raw()).unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    super::PayloadTransformSyncView::new(::protobuf::__internal::Private, inner)
  }
  pub fn transform_sync_mut(&mut self) -> super::PayloadTransformSyncMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         6, self.arena()
       ).unwrap()
     };
     super::PayloadTransformSyncMut::from_parent(
       ::protobuf::__internal::Private,
       self.as_message_mut_inner(::protobuf::__internal::Private),
       ptr.raw())
  }
  pub fn set_transform_sync(&mut self,
    val: impl ::protobuf::IntoProxied<super::PayloadTransformSync>) {

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
        6, child_ptr
      );
    }
  }

  // play_action: optional message Proto.PayloadPlayAction
  pub fn has_play_action(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn clear_play_action(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        7
      );
    }
  }
  pub fn play_action_opt(&self) -> ::protobuf::Optional<super::PayloadPlayActionView<'_>> {
        ::protobuf::Optional::new(self.play_action(), self.has_play_action())
  }
  pub fn play_action(&self) -> super::PayloadPlayActionView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(7)
    };
    let raw = submsg.map(|ptr| ptr.raw()).unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    super::PayloadPlayActionView::new(::protobuf::__internal::Private, inner)
  }
  pub fn play_action_mut(&mut self) -> super::PayloadPlayActionMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         7, self.arena()
       ).unwrap()
     };
     super::PayloadPlayActionMut::from_parent(
       ::protobuf::__internal::Private,
       self.as_message_mut_inner(::protobuf::__internal::Private),
       ptr.raw())
  }
  pub fn set_play_action(&mut self,
    val: impl ::protobuf::IntoProxied<super::PayloadPlayAction>) {

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
        7, child_ptr
      );
    }
  }

  // entity_damaged: optional message Proto.PayloadEntityDamaged
  pub fn has_entity_damaged(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(8)
    }
  }
  pub fn clear_entity_damaged(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        8
      );
    }
  }
  pub fn entity_damaged_opt(&self) -> ::protobuf::Optional<super::PayloadEntityDamagedView<'_>> {
        ::protobuf::Optional::new(self.entity_damaged(), self.has_entity_damaged())
  }
  pub fn entity_damaged(&self) -> super::PayloadEntityDamagedView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(8)
    };
    let raw = submsg.map(|ptr| ptr.raw()).unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    super::PayloadEntityDamagedView::new(::protobuf::__internal::Private, inner)
  }
  pub fn entity_damaged_mut(&mut self) -> super::PayloadEntityDamagedMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         8, self.arena()
       ).unwrap()
     };
     super::PayloadEntityDamagedMut::from_parent(
       ::protobuf::__internal::Private,
       self.as_message_mut_inner(::protobuf::__internal::Private),
       ptr.raw())
  }
  pub fn set_entity_damaged(&mut self,
    val: impl ::protobuf::IntoProxied<super::PayloadEntityDamaged>) {

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
        8, child_ptr
      );
    }
  }

  // enemy_spawn: optional message Proto.PayloadEnemySpawn
  pub fn has_enemy_spawn(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(9)
    }
  }
  pub fn clear_enemy_spawn(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        9
      );
    }
  }
  pub fn enemy_spawn_opt(&self) -> ::protobuf::Optional<super::PayloadEnemySpawnView<'_>> {
        ::protobuf::Optional::new(self.enemy_spawn(), self.has_enemy_spawn())
  }
  pub fn enemy_spawn(&self) -> super::PayloadEnemySpawnView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(9)
    };
    let raw = submsg.map(|ptr| ptr.raw()).unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    super::PayloadEnemySpawnView::new(::protobuf::__internal::Private, inner)
  }
  pub fn enemy_spawn_mut(&mut self) -> super::PayloadEnemySpawnMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         9, self.arena()
       ).unwrap()
     };
     super::PayloadEnemySpawnMut::from_parent(
       ::protobuf::__internal::Private,
       self.as_message_mut_inner(::protobuf::__internal::Private),
       ptr.raw())
  }
  pub fn set_enemy_spawn(&mut self,
    val: impl ::protobuf::IntoProxied<super::PayloadEnemySpawn>) {

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
        9, child_ptr
      );
    }
  }

  // enemy_despawn: optional message Proto.PayloadEnemyDespawn
  pub fn has_enemy_despawn(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(10)
    }
  }
  pub fn clear_enemy_despawn(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        10
      );
    }
  }
  pub fn enemy_despawn_opt(&self) -> ::protobuf::Optional<super::PayloadEnemyDespawnView<'_>> {
        ::protobuf::Optional::new(self.enemy_despawn(), self.has_enemy_despawn())
  }
  pub fn enemy_despawn(&self) -> super::PayloadEnemyDespawnView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(10)
    };
    let raw = submsg.map(|ptr| ptr.raw()).unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    super::PayloadEnemyDespawnView::new(::protobuf::__internal::Private, inner)
  }
  pub fn enemy_despawn_mut(&mut self) -> super::PayloadEnemyDespawnMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         10, self.arena()
       ).unwrap()
     };
     super::PayloadEnemyDespawnMut::from_parent(
       ::protobuf::__internal::Private,
       self.as_message_mut_inner(::protobuf::__internal::Private),
       ptr.raw())
  }
  pub fn set_enemy_despawn(&mut self,
    val: impl ::protobuf::IntoProxied<super::PayloadEnemyDespawn>) {

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
        10, child_ptr
      );
    }
  }

  // text_message: optional message Proto.PayloadTextMessage
  pub fn has_text_message(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(11)
    }
  }
  pub fn clear_text_message(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        11
      );
    }
  }
  pub fn text_message_opt(&self) -> ::protobuf::Optional<super::PayloadTextMessageView<'_>> {
        ::protobuf::Optional::new(self.text_message(), self.has_text_message())
  }
  pub fn text_message(&self) -> super::PayloadTextMessageView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(11)
    };
    let raw = submsg.map(|ptr| ptr.raw()).unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    super::PayloadTextMessageView::new(::protobuf::__internal::Private, inner)
  }
  pub fn text_message_mut(&mut self) -> super::PayloadTextMessageMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         11, self.arena()
       ).unwrap()
     };
     super::PayloadTextMessageMut::from_parent(
       ::protobuf::__internal::Private,
       self.as_message_mut_inner(::protobuf::__internal::Private),
       ptr.raw())
  }
  pub fn set_text_message(&mut self,
    val: impl ::protobuf::IntoProxied<super::PayloadTextMessage>) {

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
        11, child_ptr
      );
    }
  }

  // system_message: optional message Proto.PayloadSystemMessage
  pub fn has_system_message(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(12)
    }
  }
  pub fn clear_system_message(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        12
      );
    }
  }
  pub fn system_message_opt(&self) -> ::protobuf::Optional<super::PayloadSystemMessageView<'_>> {
        ::protobuf::Optional::new(self.system_message(), self.has_system_message())
  }
  pub fn system_message(&self) -> super::PayloadSystemMessageView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(12)
    };
    let raw = submsg.map(|ptr| ptr.raw()).unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    super::PayloadSystemMessageView::new(::protobuf::__internal::Private, inner)
  }
  pub fn system_message_mut(&mut self) -> super::PayloadSystemMessageMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         12, self.arena()
       ).unwrap()
     };
     super::PayloadSystemMessageMut::from_parent(
       ::protobuf::__internal::Private,
       self.as_message_mut_inner(::protobuf::__internal::Private),
       ptr.raw())
  }
  pub fn set_system_message(&mut self,
    val: impl ::protobuf::IntoProxied<super::PayloadSystemMessage>) {

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
        12, child_ptr
      );
    }
  }

  // zone_enter_request: optional message Proto.PayloadZoneEnterRequest
  pub fn has_zone_enter_request(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(13)
    }
  }
  pub fn clear_zone_enter_request(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        13
      );
    }
  }
  pub fn zone_enter_request_opt(&self) -> ::protobuf::Optional<super::PayloadZoneEnterRequestView<'_>> {
        ::protobuf::Optional::new(self.zone_enter_request(), self.has_zone_enter_request())
  }
  pub fn zone_enter_request(&self) -> super::PayloadZoneEnterRequestView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(13)
    };
    let raw = submsg.map(|ptr| ptr.raw()).unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    super::PayloadZoneEnterRequestView::new(::protobuf::__internal::Private, inner)
  }
  pub fn zone_enter_request_mut(&mut self) -> super::PayloadZoneEnterRequestMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         13, self.arena()
       ).unwrap()
     };
     super::PayloadZoneEnterRequestMut::from_parent(
       ::protobuf::__internal::Private,
       self.as_message_mut_inner(::protobuf::__internal::Private),
       ptr.raw())
  }
  pub fn set_zone_enter_request(&mut self,
    val: impl ::protobuf::IntoProxied<super::PayloadZoneEnterRequest>) {

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
        13, child_ptr
      );
    }
  }

  // zone_enter_response: optional message Proto.PayloadZoneEnterResponse
  pub fn has_zone_enter_response(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(14)
    }
  }
  pub fn clear_zone_enter_response(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        14
      );
    }
  }
  pub fn zone_enter_response_opt(&self) -> ::protobuf::Optional<super::PayloadZoneEnterResponseView<'_>> {
        ::protobuf::Optional::new(self.zone_enter_response(), self.has_zone_enter_response())
  }
  pub fn zone_enter_response(&self) -> super::PayloadZoneEnterResponseView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(14)
    };
    let raw = submsg.map(|ptr| ptr.raw()).unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    super::PayloadZoneEnterResponseView::new(::protobuf::__internal::Private, inner)
  }
  pub fn zone_enter_response_mut(&mut self) -> super::PayloadZoneEnterResponseMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         14, self.arena()
       ).unwrap()
     };
     super::PayloadZoneEnterResponseMut::from_parent(
       ::protobuf::__internal::Private,
       self.as_message_mut_inner(::protobuf::__internal::Private),
       ptr.raw())
  }
  pub fn set_zone_enter_response(&mut self,
    val: impl ::protobuf::IntoProxied<super::PayloadZoneEnterResponse>) {

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
        14, child_ptr
      );
    }
  }

  // zone_exit: optional message Proto.PayloadZoneExit
  pub fn has_zone_exit(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(15)
    }
  }
  pub fn clear_zone_exit(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        15
      );
    }
  }
  pub fn zone_exit_opt(&self) -> ::protobuf::Optional<super::PayloadZoneExitView<'_>> {
        ::protobuf::Optional::new(self.zone_exit(), self.has_zone_exit())
  }
  pub fn zone_exit(&self) -> super::PayloadZoneExitView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(15)
    };
    let raw = submsg.map(|ptr| ptr.raw()).unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    super::PayloadZoneExitView::new(::protobuf::__internal::Private, inner)
  }
  pub fn zone_exit_mut(&mut self) -> super::PayloadZoneExitMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         15, self.arena()
       ).unwrap()
     };
     super::PayloadZoneExitMut::from_parent(
       ::protobuf::__internal::Private,
       self.as_message_mut_inner(::protobuf::__internal::Private),
       ptr.raw())
  }
  pub fn set_zone_exit(&mut self,
    val: impl ::protobuf::IntoProxied<super::PayloadZoneExit>) {

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
        15, child_ptr
      );
    }
  }

  pub fn message(&self) -> super::packet::MessageOneof<'_> {
    match &self.message_case() {
      super::packet::MessageCase::LoginRequest =>
          super::packet::MessageOneof::LoginRequest(self.login_request()),
      super::packet::MessageCase::LoginResult =>
          super::packet::MessageOneof::LoginResult(self.login_result()),
      super::packet::MessageCase::LoginNotification =>
          super::packet::MessageOneof::LoginNotification(self.login_notification()),
      super::packet::MessageCase::LogoutRequest =>
          super::packet::MessageOneof::LogoutRequest(self.logout_request()),
      super::packet::MessageCase::LogoutResponse =>
          super::packet::MessageOneof::LogoutResponse(self.logout_response()),
      super::packet::MessageCase::LogoutNotification =>
          super::packet::MessageOneof::LogoutNotification(self.logout_notification()),
      super::packet::MessageCase::TransformSync =>
          super::packet::MessageOneof::TransformSync(self.transform_sync()),
      super::packet::MessageCase::PlayAction =>
          super::packet::MessageOneof::PlayAction(self.play_action()),
      super::packet::MessageCase::EntityDamaged =>
          super::packet::MessageOneof::EntityDamaged(self.entity_damaged()),
      super::packet::MessageCase::EnemySpawn =>
          super::packet::MessageOneof::EnemySpawn(self.enemy_spawn()),
      super::packet::MessageCase::EnemyDespawn =>
          super::packet::MessageOneof::EnemyDespawn(self.enemy_despawn()),
      super::packet::MessageCase::TextMessage =>
          super::packet::MessageOneof::TextMessage(self.text_message()),
      super::packet::MessageCase::SystemMessage =>
          super::packet::MessageOneof::SystemMessage(self.system_message()),
      super::packet::MessageCase::ZoneEnterRequest =>
          super::packet::MessageOneof::ZoneEnterRequest(self.zone_enter_request()),
      super::packet::MessageCase::ZoneEnterResponse =>
          super::packet::MessageOneof::ZoneEnterResponse(self.zone_enter_response()),
      super::packet::MessageCase::ZoneExit =>
          super::packet::MessageOneof::ZoneExit(self.zone_exit()),
      _ => super::packet::MessageOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn message_case(&self) -> super::packet::MessageCase {
    let field_num = unsafe {
      let f = ::protobuf::__internal::runtime::upb_MiniTable_GetFieldByIndex(
          <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
          0);
      ::protobuf::__internal::runtime::upb_Message_WhichOneofFieldNumber(
            self.raw_msg(), f)
    };
    unsafe {
      super::packet::MessageCase::try_from(field_num).unwrap_unchecked()
    }
  }
}  // impl Packet

impl ::std::ops::Drop for Packet {
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Packet {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Packet {
  type Proxied = Self;
  fn as_view(&self) -> PacketView {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Packet {
  type MutProxied = Self;
  fn as_mut(&mut self) -> PacketMut {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Packet {
  fn mini_table() -> *const ::protobuf::__internal::runtime::upb_MiniTable {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTablePtr> =
        ::std::sync::OnceLock::new();
    ONCE_LOCK.get_or_init(|| unsafe {
      super::Proto__Packet_msg_init.0 =
          ::protobuf::__internal::runtime::upb_MiniTable_Build(
              "$3333333333333333^!|#|$|%|&|(|)|*|+|,|-|.|/|0|1|2".as_ptr(),
              49,
              ::protobuf::__internal::runtime::THREAD_LOCAL_ARENA.with(|a| a.raw()),
              ::std::ptr::null_mut());
      let submessages = [
        <super::PayloadLoginRequest as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
        <super::PayloadLoginResult as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
        <super::PayloadLoginNotification as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
        <super::PayloadLogoutRequest as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
        <super::PayloadLogoutResponse as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
        <super::PayloadLogoutNotification as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
        <super::PayloadTransformSync as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
        <super::PayloadPlayAction as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
        <super::PayloadEntityDamaged as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
        <super::PayloadEnemySpawn as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
        <super::PayloadEnemyDespawn as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
        <super::PayloadTextMessage as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
        <super::PayloadSystemMessage as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
        <super::PayloadZoneEnterRequest as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
        <super::PayloadZoneEnterResponse as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
        <super::PayloadZoneExit as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
      ];
      let subenums = [
      ];
      assert!(::protobuf::__internal::runtime::upb_MiniTable_Link(
          super::Proto__Packet_msg_init.0,
          submessages.as_ptr() as *const *const ::protobuf::__internal::runtime::upb_MiniTable,
          submessages.len(), subenums.as_ptr(), subenums.len()));
      ::protobuf::__internal::runtime::MiniTablePtr(super::Proto__Packet_msg_init.0)
    }).0
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Packet {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for PacketView<'_> {
  #[inline(always)]
  fn mini_table() -> *const ::protobuf::__internal::runtime::upb_MiniTable {
    <Packet as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for PacketMut<'_> {
  #[inline(always)]
  fn mini_table() -> *const ::protobuf::__internal::runtime::upb_MiniTable {
    <Packet as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Packet {
  type Msg = Packet;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Packet> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Packet {
  type Msg = Packet;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Packet> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for PacketMut<'_> {
  type Msg = Packet;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Packet> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PacketMut<'_> {
  type Msg = Packet;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Packet> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PacketView<'_> {
  type Msg = Packet;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Packet> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for PacketMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod packet {

#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
#[repr(u32)]
pub enum MessageOneof<'msg> {
  LoginRequest(::protobuf::View<'msg, super::super::PayloadLoginRequest>) = 1,
  LoginResult(::protobuf::View<'msg, super::super::PayloadLoginResult>) = 2,
  LoginNotification(::protobuf::View<'msg, super::super::PayloadLoginNotification>) = 3,
  LogoutRequest(::protobuf::View<'msg, super::super::PayloadLogoutRequest>) = 4,
  LogoutResponse(::protobuf::View<'msg, super::super::PayloadLogoutResponse>) = 5,
  LogoutNotification(::protobuf::View<'msg, super::super::PayloadLogoutNotification>) = 6,
  TransformSync(::protobuf::View<'msg, super::super::PayloadTransformSync>) = 7,
  PlayAction(::protobuf::View<'msg, super::super::PayloadPlayAction>) = 8,
  EntityDamaged(::protobuf::View<'msg, super::super::PayloadEntityDamaged>) = 9,
  EnemySpawn(::protobuf::View<'msg, super::super::PayloadEnemySpawn>) = 10,
  EnemyDespawn(::protobuf::View<'msg, super::super::PayloadEnemyDespawn>) = 11,
  TextMessage(::protobuf::View<'msg, super::super::PayloadTextMessage>) = 12,
  SystemMessage(::protobuf::View<'msg, super::super::PayloadSystemMessage>) = 13,
  ZoneEnterRequest(::protobuf::View<'msg, super::super::PayloadZoneEnterRequest>) = 14,
  ZoneEnterResponse(::protobuf::View<'msg, super::super::PayloadZoneEnterResponse>) = 15,
  ZoneExit(::protobuf::View<'msg, super::super::PayloadZoneExit>) = 16,

  not_set(std::marker::PhantomData<&'msg ()>) = 0
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[allow(dead_code)]
pub enum MessageCase {
  LoginRequest = 1,
  LoginResult = 2,
  LoginNotification = 3,
  LogoutRequest = 4,
  LogoutResponse = 5,
  LogoutNotification = 6,
  TransformSync = 7,
  PlayAction = 8,
  EntityDamaged = 9,
  EnemySpawn = 10,
  EnemyDespawn = 11,
  TextMessage = 12,
  SystemMessage = 13,
  ZoneEnterRequest = 14,
  ZoneEnterResponse = 15,
  ZoneExit = 16,

  not_set = 0
}

impl MessageCase {
  #[allow(dead_code)]
  pub(crate) fn try_from(v: u32) -> ::std::option::Option<MessageCase> {
    match v {
      0 => Some(MessageCase::not_set),
      1 => Some(MessageCase::LoginRequest),
      2 => Some(MessageCase::LoginResult),
      3 => Some(MessageCase::LoginNotification),
      4 => Some(MessageCase::LogoutRequest),
      5 => Some(MessageCase::LogoutResponse),
      6 => Some(MessageCase::LogoutNotification),
      7 => Some(MessageCase::TransformSync),
      8 => Some(MessageCase::PlayAction),
      9 => Some(MessageCase::EntityDamaged),
      10 => Some(MessageCase::EnemySpawn),
      11 => Some(MessageCase::EnemyDespawn),
      12 => Some(MessageCase::TextMessage),
      13 => Some(MessageCase::SystemMessage),
      14 => Some(MessageCase::ZoneEnterRequest),
      15 => Some(MessageCase::ZoneEnterResponse),
      16 => Some(MessageCase::ZoneExit),
      _ => None
    }
  }
}
}  // pub mod packet

// upb kernel doesn't support any owned message or message mut interop.
impl ::protobuf::OwnedMessageInterop for Packet {}
impl<'a> ::protobuf::MessageMutInterop<'a> for PacketMut<'a> {}

impl<'a> ::protobuf::MessageViewInterop<'a> for PacketView<'a> {
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

