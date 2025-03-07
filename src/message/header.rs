use crate::message::rcode::Rcode;

#[derive(Default, Clone)]

///  An struct that represents a Header secction from a DNS message.
///  EDIT: now added bits AD CD for DNS security extensions.
///
///                                1  1  1  1  1  1
///  0  1  2  3  4  5  6  7  8  9  0  1  2  3  4  5
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// |                      ID                       |
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// |QR|   Opcode  |AA|TC|RD|RA| Z|AD|CD|   RCODE   |
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// |                    QDCOUNT                    |
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// |                    ANCOUNT                    |
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// |                    NSCOUNT                    |
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// |                    ARCOUNT                    |
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
#[derive (PartialEq, Debug)]
pub struct Header {
    /// Id
    id: u16,

    /// Query/Response bit. Maybe change to u8. 
    /// 
    /// [RFC 1035]: https://datatracker.ietf.org/doc/html/rfc1035#section-4.1.1
    ///  
    /// QR              A one bit field that specifies whether this message is a
    ///                 query (0), or a response (1).
    qr: bool,

    /// Operation code
    op_code: u8,

    /// Flags
    aa: bool, // Authoritative Answer
    tc: bool, // TrunCation
    rd: bool, // Recursion Desired
    ra: bool, // Recursion Available
    ad: bool, // Authentic Data
    cd: bool, // Checking Disabled 

    /// Reserved Edit: Now z is just a flag
    #[allow(dead_code)]
    z: bool,

    /// Response Code
    /// 
    /// [RFC 1035]: https://datatracker.ietf.org/doc/html/rfc1035#section-4.1.1
    /// 
    /// 0               No error condition
    ///
    /// 1               Format error - The name server was
    ///                 unable to interpret the query.
    ///
    /// 2               Server failure - The name server was
    ///                 unable to process this query due to a
    ///                 problem with the name server.
    ///
    /// 3               Name Error - Meaningful only for
    ///                 responses from an authoritative name
    ///                 server, this code signifies that the
    ///                 domain name referenced in the query does
    ///                 not exist.
    ///
    /// 4               Not Implemented - The name server does
    ///                 not support the requested kind of query.
    ///
    /// 5               Refused - The name server refuses to
    ///                 perform the specified operation for
    ///                 policy reasons.  For example, a name
    ///                 server may not wish to provide the
    ///                 information to the particular requester,
    ///                 or a name server may not wish to perform
    ///                 a particular operation.
    rcode: Rcode,

    /// Counters
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
    pub fn new() -> Self {
        let header: Header = Default::default();
        header
    }

    /// Returns a Header object from a byte array representation of a DNS message header.
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
    pub fn from_bytes(bytes: &[u8]) -> Header {
        let id = ((bytes[0] as u16) << 8) | bytes[1] as u16;
        let qr = bytes[2] >> 7;
        let op_code = (bytes[2] & 0b01111000) >> 3;
        let aa = (bytes[2] & 0b00000100) >> 2;
        let tc = (bytes[2] & 0b00000010) >> 1;
        let rd = bytes[2] & 0b00000001;
        let ra = bytes[3] >> 7;

        let ad = (bytes[3] & 0b00100000) >> 5;
        let cd = (bytes[3] & 0b00010000) >> 4;
        let rcode = Rcode::from(bytes[3] & 0b00001111);

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
        header.set_ad(ad != 0);
        header.set_cd(cd != 0);
        header.set_rcode(rcode);
        header.set_qdcount(qdcount);
        header.set_ancount(ancount);
        header.set_nscount(nscount);
        header.set_arcount(arcount);

        header
    }

    /// Gets the first byte from the ID attribute.
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

    /// Returns a byte that represents the field in the DNS message.
    ///
    /// See the DNS message structure in struct documentation for more info.
    ///
    /// # Examples
    /// ```
    /// let mut header =  Header::new();
    /// header.set_qr(true);
    ///
    /// let qr_byte = header.qr_to_byte();
    /// assert_eq!(qr_byte, 0b10000000);
    /// ```
    fn qr_to_byte(&self) -> u8 {
        let qr = self.get_qr();
        let mut qr_to_byte: u8 = 0;

        if qr == true {
            qr_to_byte = 0b10000000;
        }

        qr_to_byte
    }

    /// Returns a byte that represents the field in the DNS message.
    ///
    /// See the DNS message structure in struct documentation for more info.
    fn aa_to_byte(&self) -> u8 {
        let aa = self.get_aa();

        if aa {
            return 0b00000100;
        }

        return 0u8;
    }

    /// Returns a byte that represents the field in the DNS message.
    ///
    /// See the DNS message structure in struct documentation for more info.
    fn tc_to_byte(&self) -> u8 {
        let tc = self.get_tc();

        if tc {
            return 0b00000010;
        }

        return 0u8;
    }

    /// Returns a byte that represents the field in the DNS message.
    ///
    /// See the DNS message structure in struct documentation for more info.
    fn rd_to_byte(&self) -> u8 {
        let rd = self.get_rd();

        if rd {
            return 0b00000001;
        }

        return 0u8;
    }

    /// Returns a byte that represents the field in the DNS message.
    ///
    /// See the DNS message structure in struct documentation for more info.
    fn ra_to_byte(&self) -> u8 {
        let ra = self.get_ra();

        if ra {
            return 0b10000000;
        }

        return 0u8;
    }

    /// Returns a byte that represents the field in the DNS message.
    ///
    /// See the DNS message structure in struct documentation for more info.
    fn ad_to_byte(&self) -> u8 {
        let ad = self.get_ad();

        if ad {
            return 0b00100000;
        }

        return 0u8;
    }

    /// Returns a byte that represents the field in the DNS message.
    ///
    /// See the DNS message structure in struct documentation for more info.
    fn cd_to_byte(&self) -> u8 {
        let cd = self.get_cd();

        if cd {
            return 0b00010000;
        }

        return 0u8;
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

        let ad_byte = self.ad_to_byte();
        let cd_byte = self.cd_to_byte();
        let rcode_byte = u8::from(self.get_rcode());


        let second_byte = ra_byte | ad_byte | cd_byte |  rcode_byte;

        second_byte
    }

    /// Returns a bytes array that represents the header section of a DNS message.
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

    /// Checks if the header is well-formed.
    pub fn format_check(&self)-> Result<bool, &'static str>{

        // OP CODE: A four bit field between 0-15 
        if self.op_code > 15 {
            return Err("Format Error: OP CODE");
        }

        // Z: A z flag field MUST be zero/false
        if self.z != false {
            return Err("Format Error: Z");
        }

        // RCODE: A 4 bit field between 0-15
        if u8::from(self.rcode) > 15 {
            return Err("Format Error: RCODE");
        }
        
        Ok(true)
    }
}

/// Setters
impl Header {
    /// Sets the id attribute with a value.
    pub fn set_id(&mut self, id: u16) {
        self.id = id;
    }

    /// Sets the qr attribute with a value.
    pub fn set_qr(&mut self, qr: bool) {
        self.qr = qr;
    }

    /// Sets the op_code attribute with a value.
    pub fn set_op_code(&mut self, op_code: u8) {
        self.op_code = op_code;
    }

    /// Sets the aa attribute with a value.
    pub fn set_aa(&mut self, aa: bool) {
        self.aa = aa;
    }

    /// Sets the tc attribute with a value.
    pub fn set_tc(&mut self, tc: bool) {
        self.tc = tc;
    }

    /// Sets the rd attribute with a value.
    pub fn set_rd(&mut self, rd: bool) {
        self.rd = rd;
    }

    /// Sets the ra attribute with a value.
    pub fn set_ra(&mut self, ra: bool) {
        self.ra = ra;
    }

    /// Sets the ad attribute with a value.
    pub fn set_ad(&mut self, ad: bool) {
        self.ad = ad;
    }

    /// Sets the cd attribute with a value.
    pub fn set_cd(&mut self, cd: bool) {
        self.cd = cd;
    }

    /// Sets the rcode attribute with a value.
    pub fn set_rcode(&mut self, rcode: Rcode) {
        self.rcode = rcode;
    }

    /// Sets the qdcount attribute with a value.
    pub fn set_qdcount(&mut self, qdcount: u16) {
        self.qdcount = qdcount;
    }

    /// Sets the ancount attribute with a value.
    pub fn set_ancount(&mut self, ancount: u16) {
        self.ancount = ancount;
    }

    /// Sets the nscount attribute with a value.
    pub fn set_nscount(&mut self, nscount: u16) {
        self.nscount = nscount;
    }

    /// Sets the arcount attribute with a value.
    pub fn set_arcount(&mut self, arcount: u16) {
        self.arcount = arcount;
    }
}

// Getters
impl Header {
    /// Gets the id attribute value.
    pub fn get_id(&self) -> u16 {
        self.id
    }

    /// Gets the qr attribute value.
    pub fn get_qr(&self) -> bool {
        self.qr
    }

    /// Gets the op_code attribute value.
    pub fn get_op_code(&self) -> u8 {
        self.op_code
    }

    /// Gets the aa attribute value.
    pub fn get_aa(&self) -> bool {
        self.aa
    }

    /// Gets the tc attribute value.
    pub fn get_tc(&self) -> bool {
        self.tc
    }

    /// Gets the rd attribute value.
    pub fn get_rd(&self) -> bool {
        self.rd
    }

    /// Gets the ra attribute value.
    pub fn get_ra(&self) -> bool {
        self.ra
    }

    /// Gets the ad attribute value.
    pub fn get_ad(&self) -> bool {
        self.ad
    }

    /// Gets the cd attribute value.
    pub fn get_cd(&self) -> bool {
        self.cd
    }

    /// Gets the `rcode` attribute value.
    pub fn get_rcode(&self) -> Rcode {
        self.rcode
    }

    /// Gets the qdcount attribute value.
    pub fn get_qdcount(&self) -> u16 {
        self.qdcount
    }

    /// Gets the ancount attribute value.
    pub fn get_ancount(&self) -> u16 {
        self.ancount
    }

    /// Gets the nscount attribute value.
    pub fn get_nscount(&self) -> u16 {
        self.nscount
    }

    /// Gets the arcount attribute value.
    pub fn get_arcount(&self) -> u16 {
        self.arcount
    }
}

#[cfg(test)]
mod header_test {
    use crate::message::rcode::Rcode;

    use super::Header;

    #[test]
    fn constructor_test() {
        let header = Header::new();
        assert_eq!(header.id, 0);
        assert_eq!(header.qr, false);
        assert_eq!(header.op_code, 0);
        assert_eq!(header.aa, false);
        assert_eq!(header.tc, false);
        assert_eq!(header.rd, false);
        assert_eq!(header.ra, false);

        assert_eq!(header.ad, false);
        assert_eq!(header.cd, false);
        assert_eq!(header.rcode, Rcode::NOERROR);

        assert_eq!(header.qdcount, 0);
        assert_eq!(header.ancount, 0);
        assert_eq!(header.nscount, 0);
        assert_eq!(header.arcount, 0);
    }

    #[test]
    fn set_and_get_id() {
        let mut header = Header::new();

        let mut id = header.get_id();
        assert_eq!(id, 0);

        header.set_id(5);
        id = header.get_id();
        assert_eq!(id, 5);
    }

    #[test]
    fn set_and_get_qr() {
        let mut header = Header::new();

        let mut qr = header.get_qr();
        assert_eq!(qr, false);

        header.set_qr(true);
        qr = header.get_qr();
        assert_eq!(qr, true);
    }

    #[test]
    fn set_and_get_op_code() {
        let mut header = Header::new();

        let mut op_code = header.get_op_code();
        assert_eq!(op_code, 0);

        header.set_op_code(145);
        op_code = header.get_op_code();
        assert_eq!(op_code, 145);
    }

    #[test]
    fn set_and_get_aa() {
        let mut header = Header::new();

        let mut aa = header.get_aa();
        assert_eq!(aa, false);

        header.set_aa(true);
        aa = header.get_aa();
        assert_eq!(aa, true);
    }

    #[test]
    fn set_and_get_tc() {
        let mut header = Header::new();

        let mut tc = header.get_tc();
        assert_eq!(tc, false);

        header.set_tc(true);
        tc = header.get_tc();
        assert_eq!(tc, true);
    }

    #[test]
    fn set_and_get_rd() {
        let mut header = Header::new();

        let mut rd = header.get_rd();
        assert_eq!(rd, false);

        header.set_rd(true);
        rd = header.get_rd();
        assert_eq!(rd, true);
    }

    #[test]
    fn set_and_get_ra() {
        let mut header = Header::new();

        let mut ra = header.get_ra();
        assert_eq!(ra, false);

        header.set_ra(true);
        ra = header.get_ra();
        assert_eq!(ra, true);
    }

    #[test]
    fn set_and_get_ad() {
        let mut header = Header::new();

        let mut ad = header.get_ad();
        assert_eq!(ad, false);

        header.set_ad(true);
        ad = header.get_ad();
        assert_eq!(ad, true);
    }

    #[test]
    fn set_and_get_cd() {
        let mut header = Header::new();

        let mut cd = header.get_cd();
        assert_eq!(cd, false);

        header.set_cd(true);
        cd = header.get_cd();
        assert_eq!(cd, true);
    }

    #[test]
    fn set_and_get_rcode() {
        let mut header = Header::new();

        let mut rcode = header.get_rcode();
        assert_eq!(rcode, Rcode::NOERROR);

        header.set_rcode(Rcode::SERVFAIL);
        rcode = header.get_rcode();
        assert_eq!(rcode, Rcode::SERVFAIL);
    }

    #[test]
    fn set_and_get_qdcount() {
        let mut header = Header::new();

        let mut qdcount = header.get_qdcount();
        assert_eq!(qdcount, 0);

        header.set_qdcount(1);
        qdcount = header.get_qdcount();
        assert_eq!(qdcount, 1);
    }

    #[test]
    fn set_and_get_ancount() {
        let mut header = Header::new();

        let mut ancount = header.get_ancount();
        assert_eq!(ancount, 0);

        header.set_ancount(5);
        ancount = header.get_ancount();
        assert_eq!(ancount, 5);
    }

    #[test]
    fn set_and_get_nscount() {
        let mut header = Header::new();

        let mut nscount = header.get_nscount();
        assert_eq!(nscount, 0);

        header.set_nscount(4);
        nscount = header.get_nscount();
        assert_eq!(nscount, 4);
    }

    #[test]
    fn set_and_get_arcount() {
        let mut header = Header::new();

        let mut arcount = header.get_arcount();
        assert_eq!(arcount, 0);

        header.set_arcount(12);
        arcount = header.get_arcount();
        assert_eq!(arcount, 12);
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

        header.set_ad(true);
        header.set_cd(true);
        header.set_rcode(Rcode::REFUSED);

        header.set_ancount(0b0000101010100101);

        bytes[0] = 0b00100100;
        bytes[1] = 0b10010101;
        bytes[2] = 0b10010010;
        bytes[3] = 0b00110101;
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
        bytes[3] = 0b00110101;
        bytes[6] = 0b00001010;
        bytes[7] = 0b10100101;

        let mut header = Header::new();

        header.set_id(0b0010010010010101);
        header.set_qr(true);
        header.set_op_code(2);
        header.set_tc(true);

        header.set_ad(true);
        header.set_cd(true);
        header.set_rcode(Rcode::REFUSED);

        header.set_ancount(0b0000101010100101);

        let header_from_bytes = Header::from_bytes(&bytes);

        assert_eq!(header_from_bytes.get_id(), header.get_id());
        assert_eq!(header_from_bytes.get_qr(), header.get_qr());
        assert_eq!(header_from_bytes.get_op_code(), header.get_op_code());
        assert_eq!(header_from_bytes.get_aa(), header.get_aa());
        assert_eq!(header_from_bytes.get_tc(), header.get_tc());
        assert_eq!(header_from_bytes.get_rd(), header.get_rd());
        assert_eq!(header_from_bytes.get_ra(), header.get_ra());
        assert_eq!(header_from_bytes.get_rcode(), header.get_rcode());
        assert_eq!(header_from_bytes.get_qdcount(), header.get_qdcount());
        assert_eq!(header_from_bytes.get_ancount(), header.get_ancount());
        assert_eq!(header_from_bytes.get_nscount(), header.get_nscount());
        assert_eq!(header_from_bytes.get_arcount(), header.get_arcount());
    }

    #[test]
    fn format_check_correct(){

        let bytes_header:[u8; 12] = [
            //test passes with this one
            0b10100101, 0b10010101,     // ID
            0b00010010, 0b00000000,     // flags
            0, 1,                       // QDCOUNT
            0, 1,                       // ANCOUNT 
            0, 0,                       // NSCOUNT 
            0, 0,                       // ARCOUNT
        ];

        let header = Header::from_bytes(&bytes_header);
        let result_check = header.format_check().unwrap();

        assert_eq!(result_check, true);
    }

    #[test]
    fn format_check_incorrect(){

        let bytes_header:[u8; 12] = [
            //test passes with this one
            0b10100101, 0b10010101,     // ID
            0b00010010, 0b00011000,     // flags
            0, 1,                       // QDCOUNT
            0, 1,                       // ANCOUNT 
            0, 0,                       // NSCOUNT 
            0, 0,                       // ARCOUNT
        ];

        let mut header = Header::from_bytes(&bytes_header);

        header.z = true;
        header.set_rcode(Rcode::UNKNOWN(16));

        header.set_op_code(22);

        let result_check = header.format_check();
        assert!(result_check.is_err());
    }
}
