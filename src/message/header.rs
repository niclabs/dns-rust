#[derive(Default, Clone)]
/// An struct that represents a header secction from a dns message
///
///                                1  1  1  1  1  1
///  0  1  2  3  4  5  6  7  8  9  0  1  2  3  4  5
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// |                      ID                       |
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// |QR|   Opcode  |AA|TC|RD|RA|   Z    |   RCODE   |
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// |                    QDCOUNT                    |
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// |                    ANCOUNT                    |
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// |                    NSCOUNT                    |
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// |                    ARCOUNT                    |
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
///
pub struct Header {
    // Id
    id: u16,

    // Query/Response bit
    qr: bool,

    // Operation code
    op_code: u8,

    // Flags
    aa: bool,
    tc: bool,
    rd: bool,
    ra: bool,

    // Reserved
    #[allow(dead_code)]
    z: u8,

    // Response Code
    rcode: u8,

    // Counters
    qdcount: u16,
    ancount: u16,
    nscount: u16,
    arcount: u16,
}

// Methods
impl Header {
    /// Creates a new Header with default values.
    ///
    /// # Examples
    /// ```
    /// use header::Header;
    ///
    /// let mut header = Header::new();
    /// ```
    ///
    pub fn new() -> Self {
        let header: Header = Default::default();
        header
    }

    /// Returns a Header object from a byte array representation of a dns message header.
    ///
    /// # Examples
    /// ```
    /// let mut bytes: [u8; 12] = [0; 12];
    ///
    /// bytes[0] = 0b00100100;
    /// bytes[1] = 0b10010101;
    /// bytes[2] = 0b10010010;
    /// bytes[3] = 0b00001000;
    /// bytes[6] = 0b00001010;
    /// bytes[7] = 0b10100101;
    ///
    /// let mut header = Header::new();
    ///
    /// header.set_id(0b0010010010010101);
    /// header.set_qr(true);
    /// header.set_op_code(2);
    /// header.set_tc(true);
    /// header.set_rcode(8);
    /// header.set_ancount(0b0000101010100101);
    ///
    /// let header_from_bytes = Header::from_bytes(&bytes);
    ///
    /// assert_eq!(header_from_bytes.get_id(), header.get_id());
    /// assert_eq!(header_from_bytes.get_qr(), header.get_qr());
    /// assert_eq!(header_from_bytes.get_op_code(), header.get_op_code());
    /// assert_eq!(header_from_bytes.get_tc(), header.get_tc());
    /// assert_eq!(header_from_bytes.get_rcode(), header.get_rcode());
    /// assert_eq!(header_from_bytes.get_ancount(), header.get_ancount());
    /// ```
    ///
    pub fn from_bytes(bytes: &[u8]) -> Header {
        let id = ((bytes[0] as u16) << 8) | bytes[1] as u16;
        let qr = bytes[2] >> 7;
        let op_code = (bytes[2] & 0b01111000) >> 3;
        let aa = (bytes[2] & 0b00000100) >> 2;
        let tc = (bytes[2] & 0b00000010) >> 1;
        let rd = bytes[2] & 0b00000001;
        let ra = bytes[3] >> 7;
        let rcode = bytes[3] & 0b00001111;
        let qdcount = ((bytes[4] as u16) << 8) | bytes[5] as u16;
        let ancount = ((bytes[6] as u16) << 8) | bytes[7] as u16;
        let nscount = ((bytes[8] as u16) << 8) | bytes[9] as u16;
        let arcount = ((bytes[10] as u16) << 8) | bytes[11] as u16;

        let mut header = Header::new();
        header.set_id(id);
        header.set_qr(qr != 0);
        header.set_op_code(op_code);
        header.set_aa(aa != 0);
        header.set_tc(tc != 0);
        header.set_rd(rd != 0);
        header.set_ra(ra != 0);
        header.set_rcode(rcode);
        header.set_qdcount(qdcount);
        header.set_ancount(ancount);
        header.set_nscount(nscount);
        header.set_arcount(arcount);

        header
    }

    /// Gets the first byte from the id attribute.
    fn get_first_id_byte(&self) -> u8 {
        let header_id = self.get_id();
        let first_byte = (header_id >> 8) as u8;

        first_byte
    }

    /// Gets the second byte from the id attribute.
    fn get_second_id_byte(&self) -> u8 {
        let header_id = self.get_id();
        let second_byte = header_id as u8;

        second_byte
    }

    /// Returns a byte that represents the field in the dns message.
    ///
    /// See the dns message structure in struct documentation for more info.
    ///
    /// # Examples
    /// ```
    /// let mut header =  Header::new();
    /// header.set_qr(true);
    ///
    /// let qr_byte = header.qr_to_byte();
    /// assert_eq!(qr_byte, 0b10000000);
    /// ```
    ///
    fn qr_to_byte(&self) -> u8 {
        let qr = self.get_qr();
        let mut qr_to_byte: u8 = 0;

        if qr == true {
            qr_to_byte = 0b10000000;
        }

        qr_to_byte
    }

    /// Returns a byte that represents the field in the dns message.
    ///
    /// See the dns message structure in struct documentation for more info.
    fn aa_to_byte(&self) -> u8 {
        let aa = self.get_aa();
        let mut aa_to_byte: u8 = 0;

        if aa == true {
            aa_to_byte = 0b00000100;
        }

        aa_to_byte
    }

    /// Returns a byte that represents the field in the dns message.
    ///
    /// See the dns message structure in struct documentation for more info.
    fn tc_to_byte(&self) -> u8 {
        let tc = self.get_tc();
        let mut tc_to_byte: u8 = 0;

        if tc == true {
            tc_to_byte = 0b00000010;
        }

        tc_to_byte
    }

    /// Returns a byte that represents the field in the dns message.
    ///
    /// See the dns message structure in struct documentation for more info.
    fn rd_to_byte(&self) -> u8 {
        let rd = self.get_rd();
        let mut rd_to_byte: u8 = 0;

        if rd == true {
            rd_to_byte = 0b00000001;
        }

        rd_to_byte
    }

    /// Returns a byte that represents the field in the dns message.
    ///
    /// See the dns message structure in struct documentation for more info.
    fn ra_to_byte(&self) -> u8 {
        let ra = self.get_ra();
        let mut ra_to_byte: u8 = 0;

        if ra == true {
            ra_to_byte = 0b10000000;
        }

        ra_to_byte
    }

    /// Gets the first byte from the qdcount attribute.
    fn get_first_qdcount_byte(&self) -> u8 {
        let header_qdcount = self.get_qdcount();
        let first_byte = (header_qdcount >> 8) as u8;

        first_byte
    }

    /// Gets the second byte from the qdcount attribute.
    fn get_second_qdcount_byte(&self) -> u8 {
        let header_qdcount = self.get_qdcount();
        let second_byte = header_qdcount as u8;

        second_byte
    }

    /// Gets the first byte from the ancount attribute.
    fn get_first_ancount_byte(&self) -> u8 {
        let header_ancount = self.get_ancount();
        let first_byte = (header_ancount >> 8) as u8;

        first_byte
    }

    /// Gets the second byte from the ancount attribute.
    fn get_second_ancount_byte(&self) -> u8 {
        let header_ancount = self.get_ancount();
        let second_byte = header_ancount as u8;

        second_byte
    }

    /// Gets the first byte from the nscount attribute.
    fn get_first_nscount_byte(&self) -> u8 {
        let header_nscount = self.get_nscount();
        let first_byte = (header_nscount >> 8) as u8;

        first_byte
    }

    /// Gets the second byte from the nscount attribute.
    fn get_second_nscount_byte(&self) -> u8 {
        let header_nscount = self.get_nscount();
        let second_byte = header_nscount as u8;

        second_byte
    }

    /// Gets the first byte from the arcount attribute.
    fn get_first_arcount_byte(&self) -> u8 {
        let header_arcount = self.get_arcount();
        let first_byte = (header_arcount >> 8) as u8;

        first_byte
    }

    /// Gets the second byte from the arcount attribute.
    fn get_second_arcount_byte(&self) -> u8 {
        let header_arcount = self.get_arcount();
        let second_byte = header_arcount as u8;

        second_byte
    }

    /// Gets a byte that represents the first byte of flags section.
    fn get_first_flags_byte(&self) -> u8 {
        let qr_byte = self.qr_to_byte();
        let op_code_byte = self.get_op_code() << 3;
        let aa_byte = self.aa_to_byte();
        let tc_byte = self.tc_to_byte();
        let rd_byte = self.rd_to_byte();

        let first_byte = qr_byte | op_code_byte | aa_byte | tc_byte | rd_byte;

        first_byte
    }

    /// Gets a byte that represents the second byte of flags section.
    fn get_second_flags_byte(&self) -> u8 {
        let ra_byte = self.ra_to_byte();
        let rcode_byte = self.get_rcode();

        let second_byte = ra_byte | rcode_byte;

        second_byte
    }

    /// Returns a bytes array that represents the header section of a dns message.
    ///
    /// # Examples
    /// ```
    /// let mut header = Header::new();
    /// let mut bytes: [u8; 12] = [0; 12];
    ///
    /// assert_eq!(header.to_bytes(), bytes);
    ///
    /// header.set_id(0b0010010010010101);
    /// header.set_qr(true);
    /// header.set_op_code(2);
    /// header.set_tc(true);
    /// header.set_rcode(8);
    /// header.set_ancount(0b0000101010100101);
    ///
    /// bytes[0] = 0b00100100;
    /// bytes[1] = 0b10010101;
    /// bytes[2] = 0b10010010;
    /// bytes[3] = 0b00001000;
    /// bytes[6] = 0b00001010;
    /// bytes[7] = 0b10100101;
    ///
    /// assert_eq!(header.to_bytes(), bytes);
    /// ```
    ///
    pub fn to_bytes(&self) -> [u8; 12] {
        let mut header_bytes: [u8; 12] = [0; 12];

        header_bytes[0] = self.get_first_id_byte();
        header_bytes[1] = self.get_second_id_byte();
        header_bytes[2] = self.get_first_flags_byte();
        header_bytes[3] = self.get_second_flags_byte();
        header_bytes[4] = self.get_first_qdcount_byte();
        header_bytes[5] = self.get_second_qdcount_byte();
        header_bytes[6] = self.get_first_ancount_byte();
        header_bytes[7] = self.get_second_ancount_byte();
        header_bytes[8] = self.get_first_nscount_byte();
        header_bytes[9] = self.get_second_nscount_byte();
        header_bytes[10] = self.get_first_arcount_byte();
        header_bytes[11] = self.get_second_arcount_byte();

        header_bytes
    }
}

// Setters
impl Header {
    /// Sets the id attribute with a value
    pub fn set_id(&mut self, id: u16) {
        self.id = id;
    }

    /// Sets the qr attribute with a value
    pub fn set_qr(&mut self, qr: bool) {
        self.qr = qr;
    }

    /// Sets the op_code attribute with a value
    pub fn set_op_code(&mut self, op_code: u8) {
        self.op_code = op_code;
    }

    /// Sets the aa attribute with a value
    pub fn set_aa(&mut self, aa: bool) {
        self.aa = aa;
    }

    /// Sets the tc attribute with a value
    pub fn set_tc(&mut self, tc: bool) {
        self.tc = tc;
    }

    /// Sets the rd attribute with a value
    pub fn set_rd(&mut self, rd: bool) {
        self.rd = rd;
    }

    /// Sets the ra attribute with a value
    pub fn set_ra(&mut self, ra: bool) {
        self.ra = ra;
    }

    /// Sets the rcode attribute with a value
    pub fn set_rcode(&mut self, rcode: u8) {
        self.rcode = rcode;
    }

    /// Sets the qdcount attribute with a value
    pub fn set_qdcount(&mut self, qdcount: u16) {
        self.qdcount = qdcount;
    }

    /// Sets the ancount attribute with a value
    pub fn set_ancount(&mut self, ancount: u16) {
        self.ancount = ancount;
    }

    /// Sets the nscount attribute with a value
    pub fn set_nscount(&mut self, nscount: u16) {
        self.nscount = nscount;
    }

    /// Sets the arcount attribute with a value
    pub fn set_arcount(&mut self, arcount: u16) {
        self.arcount = arcount;
    }
}

// Getters
impl Header {
    /// Gets the id attribute value
    pub fn get_id(&self) -> u16 {
        self.id
    }

    /// Gets the qr attribute value
    pub fn get_qr(&self) -> bool {
        self.qr
    }

    /// Gets the op_code attribute value
    pub fn get_op_code(&self) -> u8 {
        self.op_code
    }

    /// Gets the aa attribute value
    pub fn get_aa(&self) -> bool {
        self.aa
    }

    /// Gets the tc attribute value
    pub fn get_tc(&self) -> bool {
        self.tc
    }

    /// Gets the rd attribute value
    pub fn get_rd(&self) -> bool {
        self.rd
    }

    /// Gets the ra attribute value
    pub fn get_ra(&self) -> bool {
        self.ra
    }

    /// Gets the rcode attribute value
    pub fn get_rcode(&self) -> u8 {
        self.rcode
    }

    /// Gets the qdcount attribute value
    pub fn get_qdcount(&self) -> u16 {
        self.qdcount
    }

    /// Gets the ancount attribute value
    pub fn get_ancount(&self) -> u16 {
        self.ancount
    }

    /// Gets the nscount attribute value
    pub fn get_nscount(&self) -> u16 {
        self.nscount
    }

    /// Gets the arcount attribute value
    pub fn get_arcount(&self) -> u16 {
        self.arcount
    }
}

mod test {
    use super::Header;

    #[test]
    fn constructor_test() {
        let header = Header::new();
        assert_eq!(header.id, 0);
        assert_eq!(header.op_code, 0);
        assert_eq!(header.aa, false);
    }

    #[test]
    fn set_and_get_id_test() {
        let mut header = Header::new();

        header.set_id(5);
        let id = header.get_id();
        assert_eq!(id, 5);
    }

    #[test]
    fn set_and_get_qr_test() {
        let mut header = Header::new();

        let mut qr = header.get_qr();
        assert_eq!(qr, false);
        header.set_qr(true);
        qr = header.get_qr();
        assert_eq!(qr, true);
    }

    #[test]
    fn set_and_get_op_code_test() {
        let mut header = Header::new();

        let mut op_code = header.get_op_code();
        assert_eq!(op_code, 0);
        header.set_op_code(145);
        op_code = header.get_op_code();
        assert_eq!(op_code, 145);
    }

    #[test]
    fn set_and_get_aa_test() {
        let mut header = Header::new();

        let mut aa = header.get_aa();
        assert_eq!(aa, false);
        header.set_aa(true);
        aa = header.get_aa();
        assert_eq!(aa, true);
    }

    #[test]
    fn set_and_get_tc_test() {
        let mut header = Header::new();

        let mut tc = header.get_tc();
        assert_eq!(tc, false);
        header.set_tc(true);
        tc = header.get_tc();
        assert_eq!(tc, true);
    }

    #[test]
    fn set_and_get_rd_test() {
        let mut header = Header::new();

        let mut rd = header.get_rd();
        assert_eq!(rd, false);
        header.set_rd(true);
        rd = header.get_rd();
        assert_eq!(rd, true);
    }

    #[test]
    fn set_and_get_ra_test() {
        let mut header = Header::new();

        let mut ra = header.get_ra();
        assert_eq!(ra, false);
        header.set_ra(true);
        ra = header.get_ra();
        assert_eq!(ra, true);
    }

    #[test]
    fn set_and_get_rcode_test() {
        let mut header = Header::new();

        let mut rcode = header.get_rcode();
        assert_eq!(rcode, 0);
        header.set_rcode(127);
        rcode = header.get_rcode();
        assert_eq!(rcode, 127);
    }

    #[test]
    fn set_and_get_qdcount_test() {
        let mut header = Header::new();

        let mut qdcount = header.get_qdcount();
        assert_eq!(qdcount, 0);
        header.set_qdcount(567);
        qdcount = header.get_qdcount();
        assert_eq!(qdcount, 567);
    }

    #[test]
    fn set_and_get_ancount_test() {
        let mut header = Header::new();

        let mut ancount = header.get_ancount();
        assert_eq!(ancount, 0);
        header.set_ancount(532);
        ancount = header.get_ancount();
        assert_eq!(ancount, 532);
    }

    #[test]
    fn set_and_get_nscount_test() {
        let mut header = Header::new();

        let mut nscount = header.get_nscount();
        assert_eq!(nscount, 0);
        header.set_nscount(585);
        nscount = header.get_nscount();
        assert_eq!(nscount, 585);
    }

    #[test]
    fn set_and_get_arcount_test() {
        let mut header = Header::new();

        let mut arcount = header.get_arcount();
        assert_eq!(arcount, 0);
        header.set_arcount(745);
        arcount = header.get_arcount();
        assert_eq!(arcount, 745);
    }

    #[test]
    fn header_to_bytes_test() {
        let mut header = Header::new();
        let mut bytes: [u8; 12] = [0; 12];

        assert_eq!(header.to_bytes(), bytes);

        header.set_id(0b0010010010010101);
        header.set_qr(true);
        header.set_op_code(2);
        header.set_tc(true);
        header.set_rcode(8);
        header.set_ancount(0b0000101010100101);

        bytes[0] = 0b00100100;
        bytes[1] = 0b10010101;
        bytes[2] = 0b10010010;
        bytes[3] = 0b00001000;
        bytes[6] = 0b00001010;
        bytes[7] = 0b10100101;

        assert_eq!(header.to_bytes(), bytes);
    }

    #[test]
    fn from_bytes_test() {
        let mut bytes: [u8; 12] = [0; 12];

        bytes[0] = 0b00100100;
        bytes[1] = 0b10010101;
        bytes[2] = 0b10010010;
        bytes[3] = 0b00001000;
        bytes[6] = 0b00001010;
        bytes[7] = 0b10100101;

        let mut header = Header::new();

        header.set_id(0b0010010010010101);
        header.set_qr(true);
        header.set_op_code(2);
        header.set_tc(true);
        header.set_rcode(8);
        header.set_ancount(0b0000101010100101);

        let header_from_bytes = Header::from_bytes(&bytes);

        assert_eq!(header_from_bytes.get_id(), header.get_id());
        assert_eq!(header_from_bytes.get_qr(), header.get_qr());
        assert_eq!(header_from_bytes.get_op_code(), header.get_op_code());
        assert_eq!(header_from_bytes.get_tc(), header.get_tc());
        assert_eq!(header_from_bytes.get_rcode(), header.get_rcode());
        assert_eq!(header_from_bytes.get_ancount(), header.get_ancount());
    }
}
